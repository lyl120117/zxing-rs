use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::decoder::ErrorCorrectionLevel;
use crate::qrcode::encoder::encoder::Encoder;
use crate::qrcode::encoder::QRCode;
use crate::writer::Writer;
use crate::WriterException;

use std::collections::HashMap;
use std::str::FromStr;

pub struct QRCodeWriter {
    pub quiet_zone_size: u32,
    encoder: Encoder,
}

impl QRCodeWriter {
    pub fn new() -> Self {
        QRCodeWriter {
            quiet_zone_size: 4,
            encoder: Encoder::new(),
        }
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
                reason: String::from(format!("Can only encode QRCode, but got: {:?}", format)),
            });
        }
        if width == 0 || height == 0 {
            return Err(WriterException {
                reason: String::from(format!(
                    "Requested dimensions are too small: {}x{}",
                    width, height
                )),
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

        let code = self
            .encoder
            .encode_hints(contents, error_correction_level, hints)
            .unwrap();
        self.render_result(code, width, height, quiet_zone)
    }
}

impl QRCodeWriter {
    pub fn render_result(
        &self,
        code: QRCode,
        width: u32,
        height: u32,
        quiet_zone: u32,
    ) -> Result<BitMatrix, WriterException> {
        let output = BitMatrix::new();
        println!("==============================render_result==================================");
        Ok(output)
    }
}
