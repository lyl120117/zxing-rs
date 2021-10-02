pub mod barcode_format;
pub mod common;
pub mod datamatrix;
pub mod encode_hint_type;
pub mod multi_format_writer;
pub mod qrcode;
pub mod writer;
pub mod writer_exception;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
