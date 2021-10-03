use crate::barcode_format::BarcodeFormat;
use crate::common::bit_matrix::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::decoder::error_correction_level::ErrorCorrectionLevel;
use crate::writer::Writer;
use crate::writer_exception::WriterException;

use std::collections::HashMap;
use std::str::FromStr;

pub struct QRCodeWriter {
    pub quiet_zone_size: u32,
}

impl QRCodeWriter {
    pub fn new() -> Self {
        QRCodeWriter { quiet_zone_size: 4 }
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
        if !format.eq(&BarcodeFormat::QRCode) {
            return Err(WriterException {
                reason: String::from(
                    "Can only encode QRCode, but got: ".to_owned() + &format.to_string()[..],
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
        if hints.contains_key(&EncodeHintType::ErrorCorrection) {
            error_correction_level = ErrorCorrectionLevel::from_str(
                hints.get(&EncodeHintType::ErrorCorrection).unwrap(),
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
            quiet_zone = self.quiet_zone_size;
        }

        Ok(BitMatrix {})
    }
}
