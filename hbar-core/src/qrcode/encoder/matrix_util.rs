use crate::common::BitArray;
use crate::qrcode::decoder::{ErrorCorrectionLevel, Version};
use crate::qrcode::encoder::{ByteMatrix, MaskUtil, QRCode};

pub struct MatrixUtil;

impl MatrixUtil {
    const POSITION_DETECTION_PATTERN: [[i32; 7]; 7] = [
        [1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 1, 1, 0, 1],
        [1, 0, 1, 1, 1, 0, 1],
        [1, 0, 1, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1],
    ];
    const POSITION_ADJUSTMENT_PATTERN: [[i32; 5]; 5] = [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ];

    // From Appendix E. Table 1, JIS0510X:2004 (p 71). The table was double-checked by komatsu.
    const POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE: [[i32; 7]; 40] = [
        [-1, -1, -1, -1, -1, -1, -1],   // Version 1
        [6, 18, -1, -1, -1, -1, -1],    // Version 2
        [6, 22, -1, -1, -1, -1, -1],    // Version 3
        [6, 26, -1, -1, -1, -1, -1],    // Version 4
        [6, 30, -1, -1, -1, -1, -1],    // Version 5
        [6, 34, -1, -1, -1, -1, -1],    // Version 6
        [6, 22, 38, -1, -1, -1, -1],    // Version 7
        [6, 24, 42, -1, -1, -1, -1],    // Version 8
        [6, 26, 46, -1, -1, -1, -1],    // Version 9
        [6, 28, 50, -1, -1, -1, -1],    // Version 10
        [6, 30, 54, -1, -1, -1, -1],    // Version 11
        [6, 32, 58, -1, -1, -1, -1],    // Version 12
        [6, 34, 62, -1, -1, -1, -1],    // Version 13
        [6, 26, 46, 66, -1, -1, -1],    // Version 14
        [6, 26, 48, 70, -1, -1, -1],    // Version 15
        [6, 26, 50, 74, -1, -1, -1],    // Version 16
        [6, 30, 54, 78, -1, -1, -1],    // Version 17
        [6, 30, 56, 82, -1, -1, -1],    // Version 18
        [6, 30, 58, 86, -1, -1, -1],    // Version 19
        [6, 34, 62, 90, -1, -1, -1],    // Version 20
        [6, 28, 50, 72, 94, -1, -1],    // Version 21
        [6, 26, 50, 74, 98, -1, -1],    // Version 22
        [6, 30, 54, 78, 102, -1, -1],   // Version 23
        [6, 28, 54, 80, 106, -1, -1],   // Version 24
        [6, 32, 58, 84, 110, -1, -1],   // Version 25
        [6, 30, 58, 86, 114, -1, -1],   // Version 26
        [6, 34, 62, 90, 118, -1, -1],   // Version 27
        [6, 26, 50, 74, 98, 122, -1],   // Version 28
        [6, 30, 54, 78, 102, 126, -1],  // Version 29
        [6, 26, 52, 78, 104, 130, -1],  // Version 30
        [6, 30, 56, 82, 108, 134, -1],  // Version 31
        [6, 34, 60, 86, 112, 138, -1],  // Version 32
        [6, 30, 58, 86, 114, 142, -1],  // Version 33
        [6, 34, 62, 90, 118, 146, -1],  // Version 34
        [6, 30, 54, 78, 102, 126, 150], // Version 35
        [6, 24, 50, 76, 102, 128, 154], // Version 36
        [6, 28, 54, 80, 106, 132, 158], // Version 37
        [6, 32, 58, 84, 110, 136, 162], // Version 38
        [6, 26, 54, 82, 110, 138, 166], // Version 39
        [6, 30, 58, 86, 114, 142, 170], // Version 40
    ];

    // Type info cells at the left top corner.
    const TYPE_INFO_COORDINATES: [[i32; 2]; 15] = [
        [8, 0],
        [8, 1],
        [8, 2],
        [8, 3],
        [8, 4],
        [8, 5],
        [8, 7],
        [8, 8],
        [7, 8],
        [5, 8],
        [4, 8],
        [3, 8],
        [2, 8],
        [1, 8],
        [0, 8],
    ];

    // From Appendix D in JISX0510:2004 (p. 67)
    const VERSION_INFO_POLY: i32 = 0x1f25; // 1 1111 0010 0101

    // From Appendix C in JISX0510:2004 (p.65).
    const TYPE_INFO_POLY: i32 = 0x537;
    const TYPE_INFO_MASK_PATTERN: i32 = 0x5412;

    // Set all cells to -1.  -1 means that the cell is empty (not set yet).
    //
    // JAVAPORT: We shouldn't need to do this at all. The code should be rewritten to begin encoding
    // with the ByteMatrix initialized all to zero.
    fn clear_matrix(matrix: &mut ByteMatrix) {
        matrix.clear(-1);
    }

    // Build 2D matrix of QR Code from "dataBits" with "ecLevel", "version" and "getMaskPattern". On
    // success, store the result in "matrix" and return true.
    pub fn build_matrix(
        data_bits: &mut BitArray,
        ec_level: &ErrorCorrectionLevel,
        version: &Version,
        mask_pattern: i32,
        matrix: &mut ByteMatrix,
    ) {
        MatrixUtil::clear_matrix(matrix);
        MatrixUtil::embed_basic_patterns(version, matrix);
        // Type information appear with any version.
        MatrixUtil::embed_type_info(ec_level, mask_pattern, matrix);
        // Version info appear if version >= 7.
        MatrixUtil::maybe_embed_version_info(version, matrix);
        // Data should be embedded at end.
        MatrixUtil::embed_data_bits(data_bits, mask_pattern, matrix);
    }

    fn is_empty(value: i32) -> bool {
        value == -1
    }

    // Embed basic patterns. On success, modify the matrix and return true.
    // The basic patterns are:
    // - Position detection patterns
    // - Timing patterns
    // - Dark dot at the left bottom corner
    // - Position adjustment patterns, if need be
    fn embed_basic_patterns(version: &Version, matrix: &mut ByteMatrix) {
        // Let's get started with embedding big squares at corners.
        MatrixUtil::embed_position_detection_patterns_and_separators(matrix);
        // Then, embed the dark dot at the left bottom corner.
        MatrixUtil::embed_dark_dot_at_left_bottom_corner(matrix);

        // Position adjustment patterns appear if version >= 2.
        MatrixUtil::maybe_embed_position_adjustment_patterns(version, matrix);
        // Timing patterns should be embedded after position adj. patterns.
        MatrixUtil::embed_timing_patterns(matrix);
    }

    // Embed type information. On success, modify the matrix.
    fn embed_type_info(
        ec_level: &ErrorCorrectionLevel,
        mask_pattern: i32,
        matrix: &mut ByteMatrix,
    ) {
        let mut type_info_bits = BitArray::new();
        MatrixUtil::make_type_info_bits(ec_level, mask_pattern, &mut type_info_bits);

        for i in 0..type_info_bits.get_size() {
            // Place bits in LSB to MSB order.  LSB (least significant bit) is the last value in
            // "typeInfoBits".
            let bit = type_info_bits.get(type_info_bits.get_size() - 1 - i);

            // Type info bits at the left top corner. See 8.9 of JISX0510:2004 (p.46).
            let coordinates = MatrixUtil::TYPE_INFO_COORDINATES[i as usize];
            let x1 = coordinates[0];
            let y1 = coordinates[1];
            matrix.set(x1, y1, bit as i32);

            let x2;
            let y2;
            if i < 8 {
                // Right top corner.
                x2 = matrix.get_width() - i - 1;
                y2 = 8;
            } else {
                // Left bottom corner.
                x2 = 8;
                y2 = matrix.get_height() - 7 + (i - 8);
            }
            matrix.set(x2, y2, bit as i32)
        }
    }

    // Embed version information if need be. On success, modify the matrix and return true.
    // See 8.10 of JISX0510:2004 (p.47) for how to embed version information.
    fn maybe_embed_version_info(version: &Version, matrix: &mut ByteMatrix) {
        if version.get_version_number() < 7 {
            // Version info is necessary if version >= 7.
            return; // Don't need version info.
        }

        let mut version_info_bits = BitArray::new();
        MatrixUtil::make_version_info_bits(version, &mut version_info_bits);

        let mut bit_index = 6 * 3 - 1; // It will decrease from 17 to 0.
        for i in 0..6 {
            for j in 0..3 {
                // Place bits in LSB (least significant bit) to MSB order.
                let bit = version_info_bits.get(bit_index);
                bit_index -= 1;
                // Left bottom corner.
                matrix.set(i, matrix.get_height() - 11 + j, bit as i32);
                // Right bottom corner.
                matrix.set(matrix.get_height() - 11 + j, i, bit as i32);
            }
        }
    }

    // Embed "dataBits" using "getMaskPattern". On success, modify the matrix and return true.
    // For debugging purposes, it skips masking process if "getMaskPattern" is -1.
    // See 8.7 of JISX0510:2004 (p.38) for how to embed data bits.
    fn embed_data_bits(data_bits: &mut BitArray, mask_pattern: i32, matrix: &mut ByteMatrix) {
        let mut bit_index = 0;
        let mut direction = -1;
        // Start from the right bottom cell.
        let mut x = matrix.get_width() - 1;
        let mut y = matrix.get_height() - 1;
        while x > 0 {
            // Skip the vertical timing pattern.
            if x == 6 {
                x -= 1;
            }
            while y >= 0 && y < matrix.get_height() {
                for i in 0..2 {
                    let xx = x - i;
                    // Skip the cell if it's not empty.
                    if !MatrixUtil::is_empty(matrix.get(xx, y)) {
                        continue;
                    }
                    let mut bit;
                    if bit_index < data_bits.get_size() {
                        bit = data_bits.get(bit_index);
                        bit_index += 1;
                    } else {
                        // Padding bit. If there is no bit left, we'll fill the left cells with 0, as described
                        // in 8.4.9 of JISX0510:2004 (p. 24).
                        bit = false;
                    }

                    // Skip masking if mask_pattern is -1.
                    if mask_pattern != -1 && MaskUtil::get_data_mask_bit(mask_pattern, xx, y) {
                        bit = !bit;
                    }
                    matrix.set(xx, y, bit as i32);
                }
                y += direction;
            }
            direction = -direction; // Reverse the direction.
            y += direction;
            x -= 2; // Move to the left.
        }
        // All bits should be consumed.
        if bit_index != data_bits.get_size() {
            panic!(
                "Not all bits consumed: {}/{}",
                bit_index,
                data_bits.get_size()
            );
        }
    }

    fn embed_timing_patterns(matrix: &mut ByteMatrix) {
        // -8 is for skipping position detection patterns (size 7), and two horizontal/vertical
        // separation patterns (size 1). Thus, 8 = 7 + 1.
        for i in 8..matrix.get_width() - 8 {
            let bit = (i + 1) % 2;
            // Horizontal line.
            if MatrixUtil::is_empty(matrix.get(i, 6)) {
                matrix.set(i, 6, bit);
            }
            // Vertical line.
            if MatrixUtil::is_empty(matrix.get(6, i)) {
                matrix.set(6, i, bit);
            }
        }
    }

    // Embed the lonely dark dot at left bottom corner. JISX0510:2004 (p.46)
    fn embed_dark_dot_at_left_bottom_corner(matrix: &mut ByteMatrix) {
        if matrix.get(8, matrix.get_height() - 8) == 0 {
            panic!()
        }
        matrix.set(8, matrix.get_height() - 8, 1);
    }

    fn embed_horizontal_separation_pattern(x_start: i32, y_start: i32, matrix: &mut ByteMatrix) {
        for x in 0..8 {
            if !MatrixUtil::is_empty(matrix.get(x_start + x, y_start)) {
                panic!()
            }
            matrix.set(x_start + x, y_start, 0)
        }
    }

    fn embed_vertical_separation_pattern(x_start: i32, y_start: i32, matrix: &mut ByteMatrix) {
        for y in 0..7 {
            if !MatrixUtil::is_empty(matrix.get(x_start, y_start + y)) {
                panic!()
            }
            matrix.set(x_start, y_start + y, 0)
        }
    }

    fn embed_position_adjustment_pattern(x_start: i32, y_start: i32, matrix: &mut ByteMatrix) {
        for y in 0..5 {
            let pattern_y = MatrixUtil::POSITION_ADJUSTMENT_PATTERN[y as usize];
            for x in 0..5 {
                matrix.set(x_start + x, y_start + y, pattern_y[x as usize]);
            }
        }
    }

    fn embed_position_detection_pattern(x_start: i32, y_start: i32, matrix: &mut ByteMatrix) {
        for y in 0..7 {
            let pattern_y = MatrixUtil::POSITION_DETECTION_PATTERN[y as usize];
            for x in 0..7 {
                matrix.set(x_start + x, y_start + y, pattern_y[x as usize]);
            }
        }
    }

    // Embed position detection patterns and surrounding vertical/horizontal separators.
    fn embed_position_detection_patterns_and_separators(matrix: &mut ByteMatrix) {
        // Embed three big squares at corners.
        let pdp_width = MatrixUtil::POSITION_DETECTION_PATTERN[0].len() as i32;
        // Left top corner.
        MatrixUtil::embed_position_detection_pattern(0, 0, matrix);
        // Right top corner.
        MatrixUtil::embed_position_detection_pattern(
            (matrix.get_width() - pdp_width) as i32,
            0,
            matrix,
        );
        // Left bottom corner.
        MatrixUtil::embed_position_detection_pattern(
            0,
            (matrix.get_width() - pdp_width) as i32,
            matrix,
        );

        // Embed horizontal separation patterns around the squares.
        let hsp_width = 8;
        // Left top corner.
        MatrixUtil::embed_horizontal_separation_pattern(0, hsp_width - 1, matrix);
        // Right top corner.
        MatrixUtil::embed_horizontal_separation_pattern(
            matrix.get_width() as i32 - hsp_width,
            hsp_width - 1,
            matrix,
        );
        // Left bottom corner.
        MatrixUtil::embed_horizontal_separation_pattern(
            0,
            matrix.get_width() as i32 - hsp_width,
            matrix,
        );

        // Embed vertical separation patterns around the squares.
        let vsp_size = 7;
        // Left top corner.
        MatrixUtil::embed_vertical_separation_pattern(vsp_size, 0, matrix);
        // Right top corner.
        MatrixUtil::embed_vertical_separation_pattern(
            matrix.get_height() - vsp_size - 1,
            0,
            matrix,
        );
        // Left bottom corner.
        MatrixUtil::embed_vertical_separation_pattern(
            vsp_size,
            matrix.get_height() - vsp_size,
            matrix,
        );
    }

    // Return the position of the most significant bit set (to one) in the "value". The most
    // significant bit is position 32. If there is no bit set, return 0. Examples:
    // - find_msb_set(0) => 0
    // - find_msb_set(1) => 1
    // - find_msb_set(255) => 8
    fn find_msb_set(value: i32) -> i32 {
        32 - i32::leading_zeros(value) as i32
    }

    // Calculate BCH (Bose-Chaudhuri-Hocquenghem) code for "value" using polynomial "poly". The BCH
    // code is used for encoding type information and version information.
    // Example: Calculation of version information of 7.
    // f(x) is created from 7.
    //   - 7 = 000111 in 6 bits
    //   - f(x) = x^2 + x^1 + x^0
    // g(x) is given by the standard (p. 67)
    //   - g(x) = x^12 + x^11 + x^10 + x^9 + x^8 + x^5 + x^2 + 1
    // Multiply f(x) by x^(18 - 6)
    //   - f'(x) = f(x) * x^(18 - 6)
    //   - f'(x) = x^14 + x^13 + x^12
    // Calculate the remainder of f'(x) / g(x)
    //         x^2
    //         __________________________________________________
    //   g(x) )x^14 + x^13 + x^12
    //         x^14 + x^13 + x^12 + x^11 + x^10 + x^7 + x^4 + x^2
    //         --------------------------------------------------
    //                              x^11 + x^10 + x^7 + x^4 + x^2
    //
    // The remainder is x^11 + x^10 + x^7 + x^4 + x^2
    // Encode it in binary: 110010010100
    // The return value is 0xc94 (1100 1001 0100)
    //
    // Since all coefficients in the polynomials are 1 or 0, we can do the calculation by bit
    // operations. We don't care if coefficients are positive or negative.
    fn calculate_bch_code(value: i32, poly: i32) -> i32 {
        if poly == 0 {
            panic!("0 polynomial")
        }
        let mut value = value;
        // If poly is "1 1111 0010 0101" (version info poly), msbSetInPoly is 13. We'll subtract 1
        // from 13 to make it 12.
        let msb_set_in_poly = MatrixUtil::find_msb_set(poly);
        // Do the division business using exclusive-or operations.
        while MatrixUtil::find_msb_set(value) >= msb_set_in_poly {
            value ^= poly << (MatrixUtil::find_msb_set(value) - msb_set_in_poly);
        }

        // Now the "value" is the remainder (i.e. the BCH code)
        value
    }

    // Make bit vector of type information. On success, store the result in "bits" and return true.
    // Encode error correction level and mask pattern. See 8.9 of
    // JISX0510:2004 (p.45) for details.
    fn make_type_info_bits(
        ec_level: &ErrorCorrectionLevel,
        mask_pattern: i32,
        bits: &mut BitArray,
    ) {
        if !QRCode::is_valid_mask_pattern(mask_pattern) {
            panic!("Invalid mask pattern")
        }
        let type_info = (ec_level.get_bits() << 3) | mask_pattern;
        bits.append_bits(type_info, 5);

        let bch_code = MatrixUtil::calculate_bch_code(type_info, MatrixUtil::TYPE_INFO_POLY);
        bits.append_bits(bch_code, 10);

        let mut mask_bits = BitArray::new();
        mask_bits.append_bits(MatrixUtil::TYPE_INFO_MASK_PATTERN, 15);
        bits.xor(&mask_bits);

        if bits.get_size() != 15 {
            // Just in case.
            panic!("should not happen but we got: {}", bits.get_size());
        }
    }

    // Make bit vector of version information. On success, store the result in "bits" and return true.
    // See 8.10 of JISX0510:2004 (p.45) for details.
    fn make_version_info_bits(version: &Version, bits: &mut BitArray) {
        bits.append_bits(version.get_version_number(), 6);
        let bch_code = MatrixUtil::calculate_bch_code(
            version.get_version_number(),
            MatrixUtil::VERSION_INFO_POLY,
        );
        bits.append_bits(bch_code, 12);

        if bits.get_size() != 18 {
            // Just in case.
            panic!("should not happen but we got: {}", bits.get_size());
        }
    }

    // Embed position adjustment patterns if need be.
    fn maybe_embed_position_adjustment_patterns(version: &Version, matrix: &mut ByteMatrix) {
        if version.get_version_number() < 2 {
            // The patterns appear if version >= 2
            return;
        }
        let index = version.get_version_number() - 1;
        let coordinates = MatrixUtil::POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE[index as usize];
        for y in coordinates {
            if y > 0 {
                for x in coordinates {
                    if x >= 0 && MatrixUtil::is_empty(matrix.get(x, y)) {
                        // If the cell is unset, we embed the position adjustment pattern here.
                        // -2 is necessary since the x/y coordinates point to the center of the pattern, not the
                        // left top corner.
                        MatrixUtil::embed_position_adjustment_pattern(x - 2, y - 2, matrix);
                    }
                }
            }
        }
    }
}
