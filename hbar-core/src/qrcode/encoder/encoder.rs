use crate::common::BitArray;
use crate::common::Charset;
use crate::common::{GenericGFEnum, ReedSolomonEncoder};
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::decoder::{ErrorCorrectionLevel, Mode, Version, Versions};
use crate::qrcode::encoder::{BlockPair, ByteMatrix, MaskUtil, MatrixUtil, QRCode};
use crate::WriterException;

use std::collections::HashMap;
use std::str::FromStr;

pub struct Encoder {
    alphanumeric_table: [i32; 96],
    default_byte_mode_encoding: Charset,
    versions: Versions,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            alphanumeric_table: [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, // 0x00-0x0f
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, // 0x10-0x1f
                36, -1, -1, -1, 37, 38, -1, -1, -1, -1, 39, 40, -1, 41, 42, 43, // 0x20-0x2f
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 44, -1, -1, -1, -1, -1, // 0x30-0x3f
                -1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, // 0x40-0x4f
                25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, -1, // 0x50-0x5f
            ],
            default_byte_mode_encoding: Charset::ISO8859_1,
            versions: Versions::new(),
        }
    }

    pub fn encode(
        &self,
        content: &String,
        ec_level: ErrorCorrectionLevel,
    ) -> Result<QRCode, WriterException> {
        let hints: HashMap<EncodeHintType, &String> = HashMap::new();
        self.encode_hints(content, ec_level, hints)
    }

    pub fn encode_hints(
        &self,
        content: &String,
        ec_level: ErrorCorrectionLevel,
        hints: HashMap<EncodeHintType, &String>,
    ) -> Result<QRCode, WriterException> {
        // Determine what character encoding has been specified by the caller, if any
        let mut encoding = self.default_byte_mode_encoding.clone();
        let has_encoding_hint = hints.contains_key(&EncodeHintType::CharacterSet);
        if has_encoding_hint {
            let tmp_charset = hints.get(&EncodeHintType::CharacterSet).unwrap();
            let tmp_encoding = Charset::from_str(tmp_charset).unwrap();
            encoding = tmp_encoding.clone();
        }

        // Pick an encoding mode appropriate for the content. Note that this will not attempt to use
        // multiple modes / segments even if that were more efficient. Twould be nice.
        let mode = self.choose_mode(content, &encoding).unwrap();
        println!("mode: {:?}", mode);

        // This will store the header information, like mode and
        // length, as well as "header" segments like an ECI segment.
        let mut header_bits = BitArray::new();

        // Append ECI segment if applicable
        if mode == Mode::get_byte() && has_encoding_hint {
            todo!("Append ECI segment if applicable")
        }

        // Append the FNC1 mode header for GS1 formatted data if applicable
        let has_gs1_format_hint = hints.contains_key(&EncodeHintType::Gs1Format);
        if has_gs1_format_hint {
            todo!("Append the FNC1 mode header for GS1 formatted data if applicable")
        }

        // (With ECI in place,) Write the mode marker
        self.append_mode_info(&mode, &mut header_bits);

        // Collect data within the main segment, separately, to count its size if needed. Don't add it to
        // main payload yet.
        let mut data_bits = BitArray::new();
        self.append_bytes(content, &mode, &mut data_bits, &encoding);

        println!("header_bits: {}", header_bits);
        println!("data_bits: {}", data_bits);

        let version;
        if hints.contains_key(&EncodeHintType::QRVersion) {
            todo!("QRVersion")
        } else {
            version = self
                .recommend_version(&ec_level, &mode, &mut header_bits, &mut data_bits)
                .unwrap();
        }
        println!("version: {:?}", version);

        let mut header_and_data_bits = BitArray::new();
        header_and_data_bits.append_bit_array(&mut header_bits);
        println!("header_bits header_and_data_bits: {}", header_and_data_bits);
        // Find "length" of main segment and write it
        let num_letters;
        if mode == Mode::get_byte() {
            num_letters = data_bits.get_size_in_bytes()
        } else {
            num_letters = content.len() as i32
        }
        self.append_length_info(num_letters, version, &mode, &mut header_and_data_bits);
        println!(
            "append_length_info header_and_data_bits: {}",
            header_and_data_bits
        );
        // Put data together into the overall payload
        header_and_data_bits.append_bit_array(&mut data_bits);
        println!("data_bits header_and_data_bits: {}", header_and_data_bits);

        let ec_blocks = version.get_ec_blocks_for_level(&ec_level);
        let num_data_bytes = version.get_total_codewords() - ec_blocks.get_total_ec_codewords();
        // Terminate the bits properly.
        self.terminate_bits(num_data_bytes, &mut header_and_data_bits);

        println!(
            "terminate_bits header_and_data_bits: {}",
            header_and_data_bits
        );

        // Interleave data bits with error correction code.
        let mut final_bits = self
            .interleave_with_ec_bytes(
                &mut header_and_data_bits,
                version.get_total_codewords(),
                num_data_bytes,
                ec_blocks.get_num_blocks(),
            )
            .unwrap();
        println!("final_bits: {}", final_bits);

        //  Choose the mask pattern and set to "qrCode".
        let dimension = version.get_dimension_for_version();
        let mut matrix = ByteMatrix::new(dimension, dimension);

        // Enable manual selection of the pattern to be used via hint
        let mut mask_pattern = -1;
        if hints.contains_key(&EncodeHintType::QRMaskPattern) {
            let hint_mask_pattern = hints.get(&EncodeHintType::QRMaskPattern).unwrap();
            let hint_mask_pattern = hint_mask_pattern.parse::<i32>().unwrap();
            if QRCode::is_valid_mask_pattern(hint_mask_pattern) {
                mask_pattern = hint_mask_pattern
            }
        }

        if mask_pattern == -1 {
            mask_pattern = self
                .choose_mask_pattern(&mut final_bits, &ec_level, &version, &mut matrix)
                .unwrap();
        }

        // Build the matrix and set it to "qrCode".
        MatrixUtil::build_matrix(
            &mut final_bits,
            &ec_level,
            version,
            mask_pattern,
            &mut matrix,
        );
        println!("mask_pattern: {}", mask_pattern);
        println!("ec_level: {:?}", ec_level);
        // println!("matrix: {}", matrix);
        let qr_code = QRCode::new(mode, ec_level, version.clone(), mask_pattern, matrix);

        Ok(qr_code)
    }

    pub fn choose_mode(
        &self,
        content: &String,
        encoding: &Charset,
    ) -> Result<Mode, WriterException> {
        if self.is_only_double_byte_kanji(content) {
            return Ok(Mode::get_kanji());
        }
        let mut has_numeric = false;
        let mut has_alphanumeric = false;
        for c in content.chars() {
            if c >= '0' && c <= '9' {
                has_numeric = true;
            } else if self._get_alphanumeric_code(c as i32) != -1 {
                has_alphanumeric = true;
            } else {
                return Ok(Mode::get_byte());
            }
        }
        if has_alphanumeric {
            return Ok(Mode::get_alphanumeric());
        }
        if has_numeric {
            return Ok(Mode::get_numeric());
        }

        Ok(Mode::get_byte())
    }

    pub fn is_only_double_byte_kanji(&self, content: &String) -> bool {
        let encoding = Charset::UTF8;
        let bytes = encoding.encode(content).unwrap();
        let length = bytes.len();
        if length % 2 != 0 {
            return false;
        }
        for b in bytes.iter() {
            let b1 = b & 0xFF;
            if (b1 < 0x81 || b1 > 0x9F) && (b1 < 0xE0 || b1 > 0xEB) {
                return false;
            }
        }
        return true;
    }

    fn _get_alphanumeric_code(&self, code: i32) -> i32 {
        if code < self.alphanumeric_table.len() as i32 {
            return self.alphanumeric_table[code as usize];
        }
        return -1;
    }

    fn append_mode_info(&self, mode: &Mode, bits: &mut BitArray) {
        bits.append_bits(mode.get_bits() as i32, 4);
    }

    fn append_bytes(&self, content: &String, mode: &Mode, bits: &mut BitArray, encoding: &Charset) {
        match mode {
            Mode::Numeric(_, _) => self.append_numeric_bytes(content, bits),
            Mode::Alphanumeric(_, _) => self.append_alphanumeric_bytes(content, bits),
            Mode::Byte(_, _) => self.append_8bit_bytes(content, bits, encoding),
            Mode::Kanji(_, _) => self.append_kanji_bytes(content, bits),
            other => {
                panic!("Invalid mode: {:?}", other);
            }
        }
    }

    fn append_numeric_bytes(&self, content: &String, bits: &mut BitArray) {
        let length = content.len();
        let mut i = 0;
        let mut chars = content.chars();
        let char0 = '0' as i32;
        while i < length {
            let num1 = chars.next().unwrap() as i32 - char0;
            if i + 2 < length {
                // Encode three numeric letters in ten bits.
                let num2 = chars.next().unwrap() as i32 - char0;
                let num3 = chars.next().unwrap() as i32 - char0;
                bits.append_bits(num1 * 100 + num2 * 10 + num3, 10);
                i += 3;
            } else if i + 1 < length {
                // Encode two numeric letters in seven bits.
                let num2 = chars.next().unwrap() as i32 - char0;
                bits.append_bits(num1 * 10 + num2, 7);
                i += 2;
            } else {
                // Encode one numeric letter in four bits.
                bits.append_bits(num1, 4);
                i += 1;
            }
        }
    }

    fn append_alphanumeric_bytes(&self, content: &String, bits: &mut BitArray) {
        todo!("append_alphanumeric_bytes")
    }

    fn append_8bit_bytes(&self, content: &String, bits: &mut BitArray, encoding: &Charset) {
        todo!("append_8bit_bytes")
    }

    fn append_kanji_bytes(&self, content: &String, bits: &mut BitArray) {
        todo!("append_kanji_bytes")
    }

    /**
     * Append length info. On success, store the result in "bits".
     */
    fn append_length_info(
        &self,
        num_letters: i32,
        version: &Version,
        mode: &Mode,
        bits: &mut BitArray,
    ) {
        let num_bits = mode.get_character_count_bits(version);
        if num_letters >= (1 << num_bits) {
            panic!("{} is bigger than {}", num_letters, (1 << num_bits) - 1);
        }
        bits.append_bits(num_letters, num_bits);
    }

    /**
     * Decides the smallest version of QR code that will contain all of the provided data.
     *
     * @throws WriterException if the data cannot fit in any version
     */
    fn recommend_version(
        &self,
        ec_level: &ErrorCorrectionLevel,
        mode: &Mode,
        header_bits: &mut BitArray,
        data_bits: &mut BitArray,
    ) -> Result<&Version, WriterException> {
        // Hard part: need to know version to know how many bits length takes. But need to know how many
        // bits it takes to know version. First we take a guess at version by assuming version will be
        // the minimum, 1:
        let version = self.versions.get_version_for_number(1).unwrap();
        let provisional_bits_needed =
            self.calculate_bits_needed(mode, header_bits, data_bits, version);
        let provisional_version = self
            .choose_version(provisional_bits_needed, ec_level)
            .unwrap();

        // Use that guess to calculate the right version. I am still not sure this works in 100% of cases.
        let bits_needed =
            self.calculate_bits_needed(mode, header_bits, data_bits, provisional_version);
        self.choose_version(bits_needed, ec_level)
    }

    fn calculate_bits_needed(
        &self,
        mode: &Mode,
        header_bits: &mut BitArray,
        data_bits: &mut BitArray,
        version: &Version,
    ) -> i32 {
        header_bits.get_size() + data_bits.get_size() + mode.get_character_count_bits(version)
    }

    fn choose_version(
        &self,
        num_input_bits: i32,
        ec_level: &ErrorCorrectionLevel,
    ) -> Result<&Version, WriterException> {
        for version_num in 1..40 {
            let version = self.versions.get_version_for_number(version_num).unwrap();
            if self.will_fit(num_input_bits, version, ec_level) {
                return Ok(version);
            }
        }
        Err(WriterException {
            reason: String::from("Data too big"),
        })
    }

    /**
     * @return true if the number of input bits will fit in a code with the specified version and
     * error correction level.
     */
    fn will_fit(
        &self,
        num_input_bits: i32,
        version: &Version,
        ec_level: &ErrorCorrectionLevel,
    ) -> bool {
        // In the following comments, we use numbers of Version 7-H.
        // num_bytes = 196
        let num_bytes = version.get_total_codewords();
        // num_ec_bytes = 130
        let ec_blocks = version.get_ec_blocks_for_level(ec_level);
        let num_ec_bytes = ec_blocks.get_total_ec_codewords();
        // num_data_bytes = 196 - 130 = 66
        let num_data_bytes = num_bytes - num_ec_bytes;
        let total_input_bytes = (num_input_bits + 7) / 8;

        num_data_bytes >= total_input_bytes
    }

    /**
     * Terminate bits as described in 8.4.8 and 8.4.9 of JISX0510:2004 (p.24).
     */
    fn terminate_bits(&self, num_data_bytes: i32, bits: &mut BitArray) {
        let capacity = num_data_bytes * 8;
        if bits.get_size() > capacity {
            panic!(
                "data bits cannot fit in the QR Code {} > {}",
                bits.get_size(),
                capacity
            )
        }
        for _ in 0..4 {
            if bits.get_size() > capacity {
                break;
            }
            bits.append_bit(false)
        }
        println!("terminate_bits11 bits: {}", bits);
        // Append termination bits. See 8.4.8 of JISX0510:2004 (p.24) for details.
        // If the last byte isn't 8-bit aligned, we'll add padding bits.
        let num_bits_in_last_byte = bits.get_size() & 0x07;
        if num_bits_in_last_byte > 0 {
            for _ in num_bits_in_last_byte..8 {
                bits.append_bit(false)
            }
        }
        println!("terminate_bits22 bits: {}", bits);
        // If we have more space, we'll fill the space with padding patterns defined in 8.4.9 (p.24).
        let num_padding_bytes = num_data_bytes - bits.get_size_in_bytes();
        for i in 0..num_padding_bytes {
            let value;
            if (i & 0x01) == 0 {
                value = 0xEC
            } else {
                value = 0x11
            }
            bits.append_bits(value, 8)
        }
        if bits.get_size() != capacity {
            panic!("Bits size does not equal capacity")
        }
    }

    /**
     * Interleave "bits" with corresponding error correction bytes. On success, store the result in
     * "result". The interleave rule is complicated. See 8.6 of JISX0510:2004 (p.37) for details.
     */
    fn interleave_with_ec_bytes(
        &self,
        bits: &mut BitArray,
        num_total_bytes: i32,
        num_data_bytes: i32,
        num_rs_blocks: i32,
    ) -> Result<BitArray, WriterException> {
        // "bits" must have "getNumDataBytes" bytes of data.
        if bits.get_size_in_bytes() != num_data_bytes {
            return Err(WriterException {
                reason: String::from("Number of bits and data bytes does not match"),
            });
        }

        // Step 1.  Divide data bytes into blocks and generate error correction bytes for them. We'll
        // store the divided data bytes blocks and error correction bytes blocks into "blocks".
        let mut data_bytes_offset: i32 = 0;
        let mut max_num_data_bytes: i32 = 0;
        let mut max_num_ec_bytes: i32 = 0;

        // Since, we know the number of reedsolmon blocks, we can initialize the vector with the number.
        let mut blocks: Vec<BlockPair> = Vec::new();
        for i in 0..num_rs_blocks {
            let mut num_data_bytes_in_block: [i32; 1] = [0; 1];
            let mut num_ec_bytes_in_block: [i32; 1] = [0; 1];
            self.get_num_data_bytes_and_num_ec_bytes_for_block_id(
                num_total_bytes,
                num_data_bytes,
                num_rs_blocks,
                i,
                &mut num_data_bytes_in_block,
                &mut num_ec_bytes_in_block,
            );

            let size = num_data_bytes_in_block[0];
            let mut data_bytes: Vec<i32> = vec![0; size as usize];
            bits.to_bytes(8 * data_bytes_offset, &mut data_bytes, 0, size);
            let ec_bytes = self.generate_ec_bytes(&mut data_bytes, num_ec_bytes_in_block[0]);
            let ec_bytes_length = ec_bytes.len() as i32;
            blocks.push(BlockPair::new(data_bytes, ec_bytes));

            max_num_data_bytes = max_num_data_bytes.max(size);
            max_num_ec_bytes = max_num_ec_bytes.max(ec_bytes_length);
            data_bytes_offset += num_data_bytes_in_block[0];
        }

        if num_data_bytes != data_bytes_offset {
            return Err(WriterException {
                reason: String::from("Data bytes does not match offset"),
            });
        }

        let mut result = BitArray::new();

        // First, place data blocks.
        for i in 0..max_num_data_bytes {
            for block in blocks.iter() {
                let data_bytes = block.get_data_bytes();
                if i < data_bytes.len() as i32 {
                    result.append_bits(data_bytes[i as usize], 8)
                }
            }
        }
        // Then, place error correction blocks.
        for i in 0..max_num_ec_bytes {
            for block in blocks.iter() {
                let ec_bytes = block.get_error_correction_bytes();
                if i < ec_bytes.len() as i32 {
                    result.append_bits(ec_bytes[i as usize], 8)
                }
            }
        }

        if num_total_bytes != result.get_size_in_bytes() {
            // Should be same.
            return Err(WriterException {
                reason: String::from(format!(
                    "Interleaving error: {} and {} differ.",
                    num_total_bytes,
                    result.get_size_in_bytes()
                )),
            });
        }

        Ok(result)
    }

    /**
     * Get number of data bytes and number of error correction bytes for block id "blockID". Store
     * the result in "numDataBytesInBlock", and "numECBytesInBlock". See table 12 in 8.5.1 of
     * JISX0510:2004 (p.30)
     */
    fn get_num_data_bytes_and_num_ec_bytes_for_block_id(
        &self,
        num_total_bytes: i32,
        num_data_bytes: i32,
        num_rs_blocks: i32,
        block_id: i32,
        num_data_bytes_in_block: &mut [i32; 1],
        num_ec_bytes_in_block: &mut [i32; 1],
    ) {
        if block_id >= num_rs_blocks {
            panic!("Block ID too large")
        }
        // num_rs_blocks_in_group2 = 196 % 5 = 1
        let num_rs_blocks_in_group2 = num_total_bytes % num_rs_blocks;
        // num_rs_blocks_in_group1 = 5 - 1 = 4
        let num_rs_blocks_in_group1 = num_rs_blocks - num_rs_blocks_in_group2;
        // num_total_bytes_in_group1 = 196 / 5 = 39
        let num_total_bytes_in_group1 = num_total_bytes / num_rs_blocks;
        // num_total_bytes_in_group2 = 39 + 1 = 40
        let num_total_bytes_in_group2 = num_total_bytes_in_group1 + 1;
        // num_data_bytes_in_group1 = 66 / 5 = 13
        let num_data_bytes_in_group1 = num_data_bytes / num_rs_blocks;
        // num_data_bytes_in_group2 = 13 + 1 = 14
        let num_data_bytes_in_group2 = num_data_bytes_in_group1 + 1;
        // numEcBytesInGroup1 = 39 - 13 = 26
        let num_ec_bytes_in_group1 = num_total_bytes_in_group1 - num_data_bytes_in_group1;
        // num_ec_bytes_in_group2 = 40 - 14 = 26
        let num_ec_bytes_in_group2 = num_total_bytes_in_group2 - num_data_bytes_in_group2;
        // Sanity checks.
        // 26 = 26
        if num_ec_bytes_in_group1 != num_ec_bytes_in_group2 {
            panic!("EC bytes mismatch")
        }
        // 5 = 4 + 1.
        if num_rs_blocks != num_rs_blocks_in_group1 + num_rs_blocks_in_group2 {
            panic!("RS blocks mismatch")
        }
        // 196 = (13 + 26) * 4 + (14 + 26) * 1
        if num_total_bytes
            != ((num_data_bytes_in_group1 + num_ec_bytes_in_group1) * num_rs_blocks_in_group1)
                + ((num_data_bytes_in_group2 + num_ec_bytes_in_group2) * num_rs_blocks_in_group2)
        {
            panic!("Total bytes mismatch")
        }
        if block_id < num_rs_blocks_in_group1 {
            num_data_bytes_in_block[0] = num_data_bytes_in_group1;
            num_ec_bytes_in_block[0] = num_ec_bytes_in_group1;
        } else {
            num_data_bytes_in_block[0] = num_data_bytes_in_group2;
            num_ec_bytes_in_block[0] = num_ec_bytes_in_group2;
        }
    }

    fn generate_ec_bytes(&self, data_bytes: &mut Vec<i32>, num_ec_bytes_in_block: i32) -> Vec<i32> {
        let num_data_bytes = data_bytes.len();
        let mut to_encode: Vec<i32> = vec![0; num_data_bytes + num_ec_bytes_in_block as usize];
        for i in 0..num_data_bytes {
            to_encode[i] = data_bytes[i] & 0xFF;
        }

        println!(
            "generate_ec_bytes to_encode: {:?}, size: {}",
            to_encode,
            to_encode.len()
        );
        ReedSolomonEncoder::new(&GenericGFEnum::QR_CODE_FIELD_256.get())
            .unwrap()
            .encode(&mut to_encode, num_ec_bytes_in_block)
            .unwrap();

        println!(
            "generate_ec_bytes encode to_encode: {:?}, size: {}",
            to_encode,
            to_encode.len()
        );
        let mut ec_bytes: Vec<i32> = vec![0; num_ec_bytes_in_block as usize];
        for i in 0..num_ec_bytes_in_block {
            ec_bytes[i as usize] = to_encode[num_data_bytes + i as usize]
        }
        ec_bytes
    }

    fn choose_mask_pattern(
        &self,
        bits: &mut BitArray,
        ec_level: &ErrorCorrectionLevel,
        version: &Version,
        matrix: &mut ByteMatrix,
    ) -> Result<i32, WriterException> {
        let mut min_penalty = i32::MAX; // Lower penalty is better.
        let mut best_mask_pattern = -1;
        // We try all mask patterns to choose the best one.
        for mask_pattern in 0..QRCode::NUM_MASK_PATTERNS {
            MatrixUtil::build_matrix(bits, ec_level, version, mask_pattern, matrix);
            let penalty = self.calculate_mask_penalty(matrix);
            if penalty < min_penalty {
                min_penalty = penalty;
                best_mask_pattern = mask_pattern;
            }
        }
        Ok(best_mask_pattern)
    }

    // The mask penalty calculation is complicated.  See Table 21 of JISX0510:2004 (p.45) for details.
    // Basically it applies four rules and summate all penalties.
    fn calculate_mask_penalty(&self, matrix: &mut ByteMatrix) -> i32 {
        MaskUtil::apply_mask_penalty_rule1(matrix)
            + MaskUtil::apply_mask_penalty_rule2(matrix)
            + MaskUtil::apply_mask_penalty_rule3(matrix)
            + MaskUtil::apply_mask_penalty_rule4(matrix)
    }
}
