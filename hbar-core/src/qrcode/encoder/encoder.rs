use crate::common::BitArray;
use crate::common::Charset;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::decoder::{ErrorCorrectionLevel, Mode, Version, Versions};
use crate::qrcode::encoder::QRCode;
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

        // let mut version = Version::new();
        if hints.contains_key(&EncodeHintType::QRVersion) {
            todo!("QRVersion")
        } else {
            println!("QRVersion")
        }

        Ok(QRCode::new())
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
            println!("c: {}", c);
            if c >= '0' && c <= '9' {
                has_numeric = true;
            } else if self._get_alphanumeric_code(c as usize) != -1 {
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

    fn _get_alphanumeric_code(&self, code: usize) -> i32 {
        if code < self.alphanumeric_table.len() {
            return self.alphanumeric_table[code];
        }
        return -1;
    }

    fn append_mode_info(&self, mode: &Mode, bits: &mut BitArray) {
        bits.append_bits(mode.get_bits() as u32, 4);
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
        let char0 = '0' as u32;
        while i < length {
            let num1 = chars.next().unwrap() as u32 - char0;
            if i + 2 < length {
                // Encode three numeric letters in ten bits.
                let num2 = chars.next().unwrap() as u32 - char0;
                let num3 = chars.next().unwrap() as u32 - char0;
                bits.append_bits(num1 * 100 + num2 * 10 + num3, 10);
                i += 3;
            } else if i + 1 < length {
                // Encode two numeric letters in seven bits.
                let num2 = chars.next().unwrap() as u32 - char0;
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

    // /**
    //  * Decides the smallest version of QR code that will contain all of the provided data.
    //  *
    //  * @throws WriterException if the data cannot fit in any version
    //  */
    // fn recommendVersion(
    //     &self,
    //     ec_level: ErrorCorrectionLevel,
    //     mode: &Mode,
    //     header_bits: &mut BitArray,
    //     data_bits: &mut BitArray,
    // ) -> Result<Version, WriterException> {
    //     // Hard part: need to know version to know how many bits length takes. But need to know how many
    //     // bits it takes to know version. First we take a guess at version by assuming version will be
    //     // the minimum, 1:
    // }

    // fn calculateBitsNeeded(
    //     &self,
    //     mode: Mode,
    //     header_bits: &mut BitArray,
    //     data_bits: &mut BitArray,
    //     version: Version,
    // ) -> usize {

    //     header_bits.get_size() + data_bits.get_size() + mode.
    // }
}
