use crate::barcode_format::BarcodeFormat;
use crate::common::bit_matrix::BitMatrix;
use crate::datamatrix::datamatrix_writer::DataMatrixWriter;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::qrcode_writer::QRCodeWriter;
use crate::writer::Writer;
use crate::writer_exception::WriterException;
use std::collections::HashMap;

pub fn get_encoders() -> HashMap<BarcodeFormat, Box<Writer>> {
    let mut maps: HashMap<BarcodeFormat, Box<Writer>> = HashMap::new();

    maps.insert(BarcodeFormat::QR_CODE, Box::new(QRCodeWriter::new()));
    maps.insert(
        BarcodeFormat::DATA_MATRIX,
        Box::new(DataMatrixWriter::new()),
    );
    return maps;
}

pub struct MultiFormatWriter {
    pub encoders: HashMap<BarcodeFormat, Box<Writer>>,
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
        // Err(WriterException {
        //     reason: String::from("Unknown"),
        // })
    }
}
