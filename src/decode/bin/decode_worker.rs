use hbar_core::MultiFormatReader;
use hbar_core::{BinaryBitmap, BufferedImage, HybridBinarizer, Reader};
use hbar_core::{DecodeHintType, DecodeHintValue};
use hbar_core::{Error, ResultError, Results};

use crate::BufferedImageLuminanceSource;
use crate::DecoderConfig;

use std::borrow::Borrow;
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
            let results = self.decode(input, &self.hints).unwrap();
        }

        todo!()
    }

    fn decode(
        &self,
        input: &Path,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> ResultError<Vec<Results>> {
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
        let bitmap = BinaryBitmap::new(HybridBinarizer::new(source));

        if self.config.dump_black_point {
            todo!()
        }

        let multi_format_reader: MultiFormatReader<
            HybridBinarizer<BufferedImageLuminanceSource>,
            BufferedImageLuminanceSource,
        > = MultiFormatReader::new()?;
        let mut results = Vec::new();
        if self.config.borrow().multi {
            todo!()
        } else {
            results.push(multi_format_reader.decode(&bitmap)?);
        }

        todo!()
    }
}
