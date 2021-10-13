use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::writer::Writer;
use crate::ResultError;
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
        width: i32,
        height: i32,
    ) -> ResultError<BitMatrix> {
        let hints: HashMap<EncodeHintType, &String> = HashMap::new();
        self.encode_hints(contents, format, width, height, hints)
    }

    fn encode_hints(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
        hints: HashMap<EncodeHintType, &String>,
    ) -> ResultError<BitMatrix> {
        Ok(BitMatrix::new2(0, 0)?)
    }
}
