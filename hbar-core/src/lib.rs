pub mod barcode_format;
pub mod common;
pub mod datamatrix;
pub mod encode_hint_type;
pub mod multi_format_writer;
pub mod qrcode;
pub mod writer;
pub mod writer_exception;

pub use crate::barcode_format::BarcodeFormat;
pub use crate::encode_hint_type::EncodeHintType;
pub use crate::multi_format_writer::MultiFormatWriter;
pub use crate::writer_exception::WriterException;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
