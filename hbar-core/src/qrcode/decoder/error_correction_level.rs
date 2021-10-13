use strum_macros::{EnumString, EnumVariantNames, ToString};
// You need to import the trait, to have access to VARIANTS
use crate::{Error, ResultError};
use strum::VariantNames;

use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString, EnumVariantNames, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum ErrorCorrectionLevel {
    /** L = ~7% correction */
    L,
    /** M = ~15% correction */
    M,
    /** Q = ~25% correction */
    Q,
    /** H = ~30% correction */
    H,
}

impl ErrorCorrectionLevel {
    const FOR_BITS: [ErrorCorrectionLevel; 4] = [
        ErrorCorrectionLevel::L,
        ErrorCorrectionLevel::M,
        ErrorCorrectionLevel::Q,
        ErrorCorrectionLevel::H,
    ];

    pub fn ordinal(&self) -> usize {
        match self {
            ErrorCorrectionLevel::L => 0,
            ErrorCorrectionLevel::M => 1,
            ErrorCorrectionLevel::Q => 2,
            ErrorCorrectionLevel::H => 3,
        }
    }

    pub fn check(ec_level: &String) -> bool {
        let levels = ErrorCorrectionLevel::VARIANTS;
        if levels.contains(&&ec_level[..]) {
            return true;
        }
        return false;
    }

    pub fn from(ec_level: &str) -> Result<ErrorCorrectionLevel, Error> {
        match ec_level {
            "L" => Ok(ErrorCorrectionLevel::L),
            "M" => Ok(ErrorCorrectionLevel::M),
            "Q" => Ok(ErrorCorrectionLevel::Q),
            "H" => Ok(ErrorCorrectionLevel::H),
            _ => Err(Error::IllegalArgumentException(
                "Invalid error correct level".to_string(),
            )),
        }
    }

    pub fn get_bits(&self) -> i32 {
        match self {
            ErrorCorrectionLevel::L => 0x01,
            ErrorCorrectionLevel::M => 0x00,
            ErrorCorrectionLevel::Q => 0x03,
            ErrorCorrectionLevel::H => 0x02,
        }
    }

    /**
     * @param bits int containing the two bits encoding a QR Code's error correction level
     * @return ErrorCorrectionLevel representing the encoded error correction level
     */
    pub fn forBits(bits: i32) -> ResultError<Rc<ErrorCorrectionLevel>> {
        if bits < 0 || bits >= ErrorCorrectionLevel::FOR_BITS.len() as i32 {
            return Err(Error::IllegalArgumentException(String::from("")));
        }
        let ec_level = &ErrorCorrectionLevel::FOR_BITS[bits as usize];
        Ok(Rc::new(ec_level.clone()))
    }
}
