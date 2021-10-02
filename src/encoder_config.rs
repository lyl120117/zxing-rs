use hbar_core;
use hbar_core::barcode_format::BarcodeFormat;

#[derive(Debug)]
pub struct EncoderConfig {
    pub barcode_format: BarcodeFormat,
    pub image_format: String,
    pub output_file: String,
    pub width: u32,
    pub height: u32,
    pub error_correction_level: String,
    pub contents: String,
}
