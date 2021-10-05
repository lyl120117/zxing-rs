use crate::barcode_format::BarcodeFormat;
use crate::common::BitMatrix;
use crate::encode_hint_type::EncodeHintType;
use crate::WriterException;
use std::collections::HashMap;

pub trait Writer {
    /**
     * Encode a barcode using the default settings.
     *
     * @param contents The contents to encode in the barcode
     * @param format The barcode format to generate
     * @param width The preferred width in pixels
     * @param height The preferred height in pixels
     * @return {@link BitMatrix} representing encoded barcode image
     * @throws WriterException if contents cannot be encoded legally in a format
     */
    fn encode(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<BitMatrix, WriterException>;
    fn encode_hints(
        &self,
        contents: &String,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
        hints: HashMap<EncodeHintType, &String>,
    ) -> Result<BitMatrix, WriterException>;
}
