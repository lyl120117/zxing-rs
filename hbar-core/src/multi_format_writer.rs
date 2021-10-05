use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::datamatrix::DataMatrixWriter;
use crate::encode_hint_type::EncodeHintType;
use crate::qrcode::QRCodeWriter;
use crate::writer::Writer;
use crate::WriterException;

use std::collections::HashMap;

use image;

pub struct MultiFormatWriter {
    encoders: HashMap<BarcodeFormat, Box<dyn Writer>>,
}

impl MultiFormatWriter {
    pub fn get_encoders() -> HashMap<BarcodeFormat, Box<dyn Writer>> {
        let mut maps: HashMap<BarcodeFormat, Box<dyn Writer>> = HashMap::new();

        maps.insert(BarcodeFormat::QRCode, Box::new(QRCodeWriter::new()));
        maps.insert(BarcodeFormat::DataMatrix, Box::new(DataMatrixWriter::new()));
        return maps;
    }
    pub fn new() -> MultiFormatWriter {
        MultiFormatWriter {
            encoders: MultiFormatWriter::get_encoders(),
        }
    }

    pub fn write_to_path(&self, bit_matrix: &BitMatrix, path: &String) {
        println!(
            "write_to_path width: {}, height: {}, row_size: {}",
            bit_matrix.width, bit_matrix.height, bit_matrix.row_size
        );
        println!("write_to_path path: {}", path);

        let image_width = bit_matrix.width as u32;
        let image_height = bit_matrix.height as u32;
        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let bit: u8;
            if bit_matrix.get(x, y) {
                bit = 0
            } else {
                bit = 255
            }
            *pixel = image::Luma([bit]);
        }

        // Save the image as out.png”, the format is deduced from the path
        imgbuf.save(path).unwrap();
    }
}

impl Writer for MultiFormatWriter {
    fn encode(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<BitMatrix, WriterException> {
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
    ) -> Result<BitMatrix, WriterException> {
        println!("encode begining...");
        let encoder = self.encoders.get(format).unwrap();
        let resut = encoder.encode_hints(contents, format, width, height, hints);
        println!("encode end!");
        resut
    }
}
