use crate::barcode_format::BarcodeFormat;
use crate::common::bit_matrix::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::writer::Writer;
use crate::writer_exception::WriterException;
use std::collections::HashMap;
use std::fmt;

pub struct QRCodeWriter;

impl Writer for QRCodeWriter {
    fn encode(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: u32,
        height: u32,
    ) -> Result<BitMatrix, WriterException> {
        let hints: HashMap<EncodeHintType, &String> = HashMap::new();
        self.encode_hints(contents, format, width, height, hints)
    }

    fn encode_hints(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: u32,
        height: u32,
        hints: HashMap<EncodeHintType, &String>,
    ) -> Result<BitMatrix, WriterException> {
        Ok(BitMatrix {})
    }
}
