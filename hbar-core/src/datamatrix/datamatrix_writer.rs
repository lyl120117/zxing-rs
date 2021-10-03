use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::writer::Writer;
use crate::WriterException;
use std::collections::HashMap;

pub struct DataMatrixWriter;

impl DataMatrixWriter {
    pub fn new() -> Self {
        DataMatrixWriter {}
    }
}

impl Writer for DataMatrixWriter {
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
