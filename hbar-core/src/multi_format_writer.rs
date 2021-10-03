use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::datamatrix::DataMatrixWriter;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::QRCodeWriter;
use crate::writer::Writer;
use crate::WriterException;
use std::collections::HashMap;

pub fn get_encoders() -> HashMap<BarcodeFormat, Box<dyn Writer>> {
    let mut maps: HashMap<BarcodeFormat, Box<dyn Writer>> = HashMap::new();

    maps.insert(BarcodeFormat::QRCode, Box::new(QRCodeWriter::new()));
    maps.insert(BarcodeFormat::DataMatrix, Box::new(DataMatrixWriter::new()));
    return maps;
}

pub struct MultiFormatWriter {
    pub encoders: HashMap<BarcodeFormat, Box<dyn Writer>>,
}

impl Writer for MultiFormatWriter {
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
        println!("encode begining...");
        let encoder = self.encoders.get(format).unwrap();
        let resut = encoder.encode_hints(contents, format, width, height, hints);
        println!("encode end!");
        resut
    }
}
