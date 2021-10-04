use crate::qrcode::decoder::{ErrorCorrectionLevel, Mode, Version};
use crate::qrcode::encoder::ByteMatrix;

pub struct QRCode<'a> {
    mode: &'a Mode,
    ec_level: &'a ErrorCorrectionLevel,
    version: &'a Version,
    mask_pattern: i32,
    matrix: &'a ByteMatrix,
}

impl<'a> QRCode<'a> {
    pub const NUM_MASK_PATTERNS: i32 = 8;
    pub fn new(
        mode: &'a Mode,
        ec_level: &'a ErrorCorrectionLevel,
        version: &'a Version,
        mask_pattern: i32,
        matrix: &'a ByteMatrix,
    ) -> QRCode<'a> {
        let s = QRCode::NUM_MASK_PATTERNS;
        QRCode {
            mode,
            ec_level,
            version,
            mask_pattern,
            matrix,
        }
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }
    pub fn get_ec_level(&self) -> &ErrorCorrectionLevel {
        &self.ec_level
    }
    pub fn get_version(&self) -> &Version {
        &self.version
    }
    pub fn get_mask_pattern(&self) -> i32 {
        self.mask_pattern
    }
    pub fn get_matrix(&self) -> &ByteMatrix {
        &self.matrix
    }

    // Check if "mask_pattern" is valid.
    pub fn is_valid_mask_pattern(mask_pattern: i32) -> bool {
        if mask_pattern < 0 {
            return false;
        }
        mask_pattern < QRCode::NUM_MASK_PATTERNS
    }
}
