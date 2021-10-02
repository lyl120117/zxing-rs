use hbar_core;
use hbar_core::barcode_format::BarcodeFormat;
use hbar_core::encode_hint_type::EncodeHintType;
use hbar_core::multi_format_writer::{get_encoders, MultiFormatWriter};
use hbar_core::writer::Writer;

use crate::encoder_config::EncoderConfig;
use std::collections::HashMap;

pub fn encode() {
    let config = EncoderConfig {
        barcode_format: BarcodeFormat::QR_CODE,
        image_format: String::from("png"),
        output_file: String::from("test.png"),
        width: 32,
        height: 32,
        error_correction_level: String::from("Q"),
        contents: String::from("Hello World!"),
    };
    let formater = MultiFormatWriter {
        encoders: get_encoders(),
    };
    let mut hints: HashMap<EncodeHintType, &String> = HashMap::new();
    if !config.error_correction_level.is_empty() {
        hints.insert(
            EncodeHintType::ERROR_CORRECTION,
            &config.error_correction_level,
        );
    }
    let result = formater.encode_hints(
        &config.contents,
        &config.barcode_format,
        config.width,
        config.height,
        hints,
    );
    println!("{:#?}", config);
    println!("{:#?}", result.unwrap());
}
