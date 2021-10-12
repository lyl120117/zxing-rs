mod barcode_format;
mod binarizer;
mod binary_bitmap;
mod buffered_image;
mod common;
mod datamatrix;
mod decode_hint_type;
mod encode_hint_type;
mod error;
mod inverted_luminance_source;
mod luminance_source;
mod multi_format_reader;
mod multi_format_writer;
mod qrcode;
mod reader;
mod result;
mod result_metadata_type;
mod result_point;
pub mod types;
mod writer;
mod writer_exception;

pub use crate::barcode_format::BarcodeFormat;
pub use crate::binarizer::Binarizer;
pub use crate::binary_bitmap::BinaryBitmap;
pub use crate::buffered_image::BufferedImage;
pub use crate::common::HybridBinarizer;
pub use crate::decode_hint_type::{DecodeHintType, DecodeHintValue};
pub use crate::encode_hint_type::EncodeHintType;
pub use crate::error::{Error, ResultError};
pub use crate::inverted_luminance_source::InvertedLuminanceSource;
pub use crate::luminance_source::LuminanceSource;
pub use crate::multi_format_reader::MultiFormatReader;
pub use crate::multi_format_writer::MultiFormatWriter;
pub use crate::qrcode::QRCodeReader;
pub use crate::reader::Reader;
pub use crate::result::Results;
pub use crate::result_metadata_type::{ResultMetadataType, ResultMetadataValue};
pub use crate::result_point::ResultPoint;
pub use crate::writer::Writer;
pub use crate::writer_exception::WriterException;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
