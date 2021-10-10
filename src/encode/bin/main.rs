use hbar_core::EncodeHintType;
use hbar_core::MultiFormatWriter;
use hbar_core::Writer;

mod encoder_config;
use crate::encoder_config::EncoderConfig;
use std::collections::HashMap;
use structopt::StructOpt;

pub fn encode() {
    let config = EncoderConfig::from_args();
    let formater = MultiFormatWriter::new();
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
