use hbar_core::BarcodeFormat;
use hbar_core::{DecodeHintType, DecodeHintValue};

use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "decode", about = "Usage of decode.")]
pub struct DecoderConfig {
    /// Use the TRY_HARDER hint, default is normal mode
    #[structopt(short, long)]
    pub try_harder: bool,

    /// Input image is a pure monochrome barcode image, not a photo
    #[structopt(short, long)]
    pub pure_barcode: bool,

    /// Only decode the UPC and EAN families of barcodes
    #[structopt(long)]
    pub products_only: bool,

    /// Write the decoded contents to input.txt
    #[structopt(short, long)]
    pub dump_results: bool,

    /// Compare black point algorithms with dump as input.mono.png
    #[structopt(long)]
    pub dump_black_point: bool,

    /// Scans image for multiple barcodes
    #[structopt(short, long)]
    pub multi: bool,

    /// Only output one line per file, omitting the contents
    #[structopt(short, long)]
    pub brief: bool,

    /// Output raw bitstream, before decoding symbols
    #[structopt(long)]
    pub output_raw: bool,

    /// Descend into subdirectories
    #[structopt(short, long)]
    pub recursive: bool,

    /// Only examine cropped region of input image(s)
    #[structopt(short, long)]
    pub crop: Vec<i32>,

    /// Formats to decode, where format is any value in BarcodeFormat
    #[structopt(long)]
    pub possible_formats: Vec<BarcodeFormat>,

    /// (URIs to decode)
    #[structopt(long)]
    pub input_paths: Vec<String>,
}

impl DecoderConfig {
    pub fn build_hints<'a>(&self) -> HashMap<DecodeHintType, DecodeHintValue> {
        let mut final_possible_formats = Vec::new();
        final_possible_formats.push(BarcodeFormat::UpcA);
        final_possible_formats.push(BarcodeFormat::UpcE);
        final_possible_formats.push(BarcodeFormat::Ean13);
        final_possible_formats.push(BarcodeFormat::Ean8);
        final_possible_formats.push(BarcodeFormat::RSS14);
        final_possible_formats.push(BarcodeFormat::RssExpanded);
        if !self.products_only {
            final_possible_formats.push(BarcodeFormat::Code39);
            final_possible_formats.push(BarcodeFormat::Code93);
            final_possible_formats.push(BarcodeFormat::Code128);
            final_possible_formats.push(BarcodeFormat::ITF);
            final_possible_formats.push(BarcodeFormat::QRCode);
            final_possible_formats.push(BarcodeFormat::DataMatrix);
            final_possible_formats.push(BarcodeFormat::Aztec);
            final_possible_formats.push(BarcodeFormat::PDF417);
            final_possible_formats.push(BarcodeFormat::CodeBar);
            final_possible_formats.push(BarcodeFormat::MaxiCode);
        }
        let mut hints = HashMap::new();
        hints.insert(
            DecodeHintType::PossibleFormats,
            DecodeHintValue::VecBarcodeFormat(final_possible_formats),
        );
        if self.try_harder {
            hints.insert(DecodeHintType::TryHarder, DecodeHintValue::BOOL(true));
        }

        if self.pure_barcode {
            hints.insert(DecodeHintType::PureBarcode, DecodeHintValue::BOOL(true));
        }

        hints
    }
}
