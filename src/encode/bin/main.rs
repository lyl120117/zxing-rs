use hbar_core;
use hbar_core::encode_hint_type::EncodeHintType;
use hbar_core::multi_format_writer::{get_encoders, MultiFormatWriter};
use hbar_core::writer::Writer;

mod encoder_config;
use crate::encoder_config::EncoderConfig;
use std::collections::HashMap;
use structopt::StructOpt;

pub fn encode() {
    let config = EncoderConfig::from_args();
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

fn main() {
    encode();
}