use crate::barcode_format::BarcodeFormat;
use crate::common::bit_matrix::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::decoder::error_correction_level::ErrorCorrectionLevel;
use crate::writer::Writer;
use crate::writer_exception::WriterException;

use std::collections::HashMap;
use std::str::FromStr;

pub struct QRCodeWriter {
    pub QUIET_ZONE_SIZE: u32,
}

impl QRCodeWriter {
    pub fn new() -> Self {
        QRCodeWriter { QUIET_ZONE_SIZE: 4 }
    }
}

impl Writer for QRCodeWriter {
    fn encode(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: u32,
        height: u32,
    ) -> Result<BitMatrix, WriterException> {
        let hints: HashMap<EncodeHintType, &String> = HashMap::new();
        self.encode_hints(contents, format, width, height, hints)
    }

    fn encode_hints(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: u32,
        height: u32,
        hints: HashMap<EncodeHintType, &String>,
    ) -> Result<BitMatrix, WriterException> {
        if contents.is_empty() {
            return Err(WriterException {
                reason: String::from("Found empty contents."),
            });
        }
        if !format.eq(&BarcodeFormat::QR_CODE) {
            return Err(WriterException {
                reason: String::from(
                    "Can only encode QR_CODE, but got: ".to_owned() + &format.to_string()[..],
                ),
            });
        }
        if width == 0 || height == 0 {
            return Err(WriterException {
                reason: String::from(
                    "Requested dimensions are too small: ".to_owned() + &width.to_string()[..],
                ) + &"x".to_owned()[..]
                    + &height.to_string()[..],
            });
        }

        let error_correction_level;
        if hints.contains_key(&EncodeHintType::ERROR_CORRECTION) {
            error_correction_level = ErrorCorrectionLevel::from_str(
                hints.get(&EncodeHintType::ERROR_CORRECTION).unwrap(),
            )
            .unwrap();
        } else {
            error_correction_level = ErrorCorrectionLevel::L;
        }

        let quiet_zone;
        if hints.contains_key(&EncodeHintType::MARGIN) {
            let srt_num = hints.get(&EncodeHintType::MARGIN).unwrap();

            quiet_zone = srt_num.parse::<u32>().unwrap();
        } else {
            quiet_zone = self.QUIET_ZONE_SIZE;
        }

        Ok(BitMatrix {})
    }
}
