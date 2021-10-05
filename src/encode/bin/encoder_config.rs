use hbar_core;
use hbar_core::barcode_format::BarcodeFormat;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "zxing-rs", about = "Usage of zxing-rs.")]
pub struct EncoderConfig {
    /// Format to encode, from BarcodeFormat class. Not all formats are supported
    #[structopt(short = "b", parse(try_from_str), default_value = "QRCode")]
    pub barcode_format: BarcodeFormat,
    /// Image output format, such as PNG, JPG, GIF
    #[structopt(short = "f", default_value = "PNG")]
    pub image_format: String,
    /// File to write to. Defaults to out.png
    #[structopt(short = "o", default_value = "out")]
    pub output_file_base: String,
    /// Image width
    #[structopt(short = "w", default_value = "300")]
    pub width: i32,
    /// Image height
    #[structopt(short = "h", default_value = "300")]
    pub height: i32,
    /// Error correction level for the encoding
    #[structopt(short = "e", default_value = "None")]
    pub error_correction_level: String,
    /// Text to encode
    #[structopt(parse(from_str), default_value = "Hello World!")]
    pub contents: String,
}
