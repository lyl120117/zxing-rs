use crate::qrcode::decoder::ErrorCorrectionLevel;

pub struct QRCode {
    num_mask_patterns: u32,
}

impl QRCode {
    pub fn new() -> QRCode {
        QRCode {
            num_mask_patterns: 8,
        }
    }
}
