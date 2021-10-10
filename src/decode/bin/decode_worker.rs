use hbar_core::BufferedImage;
use hbar_core::Error;
use hbar_core::Results;
use hbar_core::{DecodeHintType, DecodeHintValue};

use crate::BufferedImageLuminanceSource;
use crate::DecoderConfig;

use std::collections::HashMap;
use std::path::Path;

pub struct DecodeWorker<'a> {
    config: DecoderConfig,
    inputs: Vec<&'a Path>,
    hints: HashMap<DecodeHintType, DecodeHintValue>,
}

impl<'a> DecodeWorker<'a> {
    const RED: u32 = 0xFFFF0000;
    const BLACK: u32 = 0xFF000000;
    const WHITE: u32 = 0xFFFFFFFF;

    pub fn new(config: DecoderConfig, inputs: Vec<&'a Path>) -> DecodeWorker {
        DecodeWorker {
            hints: config.build_hints(),
            config: config,
            inputs: inputs,
        }
    }

    pub fn call(&self) -> i32 {
        let mut successful = 0;
        for input in &self.inputs {
            println!("DecodeWorker  input: {:?}", input);
            // let results
        }

        todo!()
    }

    fn decode(
        &self,
        input: &Path,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> Result<Results, Error> {
        let image = BufferedImage::open(input)?;

        let source;
        if self.config.crop.is_empty() {
            source = BufferedImageLuminanceSource::new(&image)?
        } else {
            let crop = &self.config.crop;
            source = BufferedImageLuminanceSource::new1(
                &image,
                crop[0] as u32,
                crop[1] as u32,
                crop[2] as u32,
                crop[3] as u32,
            )?;
        }

        todo!()
    }
}
