use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::decoder::ErrorCorrectionLevel;
use crate::qrcode::encoder::{Encoder, QRCode};
use crate::writer::Writer;
use crate::WriterException;

use std::collections::HashMap;
use std::str::FromStr;

pub struct QRCodeWriter {
    encoder: Encoder,
}

impl QRCodeWriter {
    const QUIET_ZONE_SIZE: i32 = 4;
    pub fn new() -> Self {
        QRCodeWriter {
            encoder: Encoder::new(),
        }
    }
}

impl Writer for QRCodeWriter {
    fn encode(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<BitMatrix, WriterException> {
        let hints: HashMap<EncodeHintType, &String> = HashMap::new();
        self.encode_hints(contents, format, width, height, hints)
    }

    fn encode_hints(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
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

            quiet_zone = srt_num.parse::<i32>().unwrap();
        } else {
            quiet_zone = QRCodeWriter::QUIET_ZONE_SIZE;
        }

        let code = self
            .encoder
            .encode_hints(contents, error_correction_level, hints)
            .unwrap();
        self.render_result(code, width, height, quiet_zone)
    }
}

impl QRCodeWriter {
    // Note that the input matrix uses 0 == white, 1 == black, while the output matrix uses
    // 0 == black, 255 == white (i.e. an 8 bit greyscale bitmap).
    pub fn render_result(
        &self,
        code: QRCode,
        width: i32,
        height: i32,
        quiet_zone: i32,
    ) -> Result<BitMatrix, WriterException> {
        let input = code.get_matrix();
        let input_width = input.get_width();
        let input_height = input.get_height();
        let qr_width = input_width + (quiet_zone * 2);
        let qr_height = input_height + (quiet_zone * 2);
        let output_width = qr_width.max(width);
        let output_height = qr_height.max(height);

        let multiple = (output_width / qr_width).min(output_height / qr_height);
        // Padding includes both the quiet zone and the extra white pixels to accommodate the requested
        // dimensions. For example, if input is 25x25 the QR will be 33x33 including the quiet zone.
        // If the requested size is 200x160, the multiple will be 4, for a QR of 132x132. These will
        // handle all the padding from 100x100 (the actual QR) up to 200x160.
        let left_padding = (output_width - (input_width * multiple)) / 2;
        let top_padding = (output_height - (input_height * multiple)) / 2;

        let mut output = BitMatrix::new2(output_width, output_height);

        let mut output_y = top_padding;
        for input_y in 0..input_height {
            let mut output_x = left_padding;
            // Write the contents of this row of the barcode
            for input_x in 0..input_width {
                if input.get(input_x, input_y) == 1 {
                    output.set_region(output_x, output_y, multiple, multiple);
                }
                output_x += multiple;
            }
            output_y += multiple;
        }
        Ok(output)
    }
}
