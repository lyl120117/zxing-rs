mod buffered_image_luminance_source;
mod decode_worker;
mod decoder_config;

use buffered_image_luminance_source::BufferedImageLuminanceSource;
use decode_worker::DecodeWorker;
use decoder_config::DecoderConfig;

use std::path::Path;
use structopt::StructOpt;

use hbar_core::ResultPoint;

fn main() {
    let config = DecoderConfig::from_args();
    println!("config: {:#?}", config);

    let mut inputs = Vec::new();
    for input_path in &config.input_paths {
        let path = Path::new(input_path);

        if !path.exists() {
            eprintln!("Couldn't find input file: {}", input_path);
            continue;
        }
        if path.is_dir() {
            todo!("Will support in future")
        }
        inputs.push(path);
    }

    let num_inputs = inputs.len();
    if num_inputs == 0 {
        eprintln!("No input files were found");
        return;
    }

    let mut num_threads = num_inputs;
    num_threads = num_threads.min(1);
    if num_threads > 1 {
        todo!("")
    } else {
        DecodeWorker::new(config.clone(), inputs).call();
    }

    println!("----------")
}
