use encoding::all::{ASCII, GBK, ISO_8859_1, UTF_8};
use encoding::{EncoderTrap, Encoding};
use strum_macros::EnumString;
use strum_macros::ToString;

use crate::WriterException;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString, Copy, Clone)]
pub enum Charset {
    ASCII,
    ISO8859_1,
    UTF8,
    GBK,
}

impl Charset {
    pub fn encode(&self, value: &String) -> Result<Vec<u8>, WriterException> {
        println!("Charset encode: {:?}, value: {}", self, value);
        match Some(self) {
            Some(Charset::ASCII) => Ok(ASCII.encode(value, EncoderTrap::Strict).unwrap()),
            Some(Charset::ISO8859_1) => Ok(ISO_8859_1.encode(value, EncoderTrap::Strict).unwrap()),
            Some(Charset::UTF8) => Ok(UTF_8.encode(value, EncoderTrap::Strict).unwrap()),
            Some(Charset::GBK) => Ok(GBK.encode(value, EncoderTrap::Strict).unwrap()),
            None => Err(WriterException {
                reason: String::from("None"),
            }),
        }
    }
}
