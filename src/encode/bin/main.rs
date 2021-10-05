use hbar_core;
use hbar_core::encode_hint_type::EncodeHintType;
use hbar_core::writer::Writer;
use hbar_core::{get_encoders, MultiFormatWriter};

mod encoder_config;
use crate::encoder_config::EncoderConfig;
use std::collections::HashMap;
use structopt::StructOpt;

pub fn encode() {
    let config = EncoderConfig::from_args();
    let formater = MultiFormatWriter {
        encoders: get_encoders(),
    };
    let out_file_string = format!(
        "{}.{}",
        &config.output_file_base,
        &config.image_format.to_lowercase()
    );

    let mut hints: HashMap<EncodeHintType, &String> = HashMap::new();
    if !config.error_correction_level.is_empty()
        && config.error_correction_level != String::from("None")
    {
        hints.insert(
            EncodeHintType::ErrorCorrection,
            &config.error_correction_level,
        );
    }
    println!("{:#?}", config);
    let result = formater
        .encode_hints(
            &config.contents,
            &config.barcode_format,
            config.width,
            config.height,
            hints,
        )
        .unwrap();
    formater.write_to_path(&result, &out_file_string);
}

fn main() {
    encode();
}
