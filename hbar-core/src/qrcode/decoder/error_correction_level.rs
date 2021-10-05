use strum_macros::{EnumString, EnumVariantNames, ToString};
// You need to import the trait, to have access to VARIANTS
use strum::VariantNames;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString, EnumVariantNames)]
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

    pub fn get_bits(&self) -> i32 {
        match self {
            ErrorCorrectionLevel::L => 0x01,
            ErrorCorrectionLevel::M => 0x00,
            ErrorCorrectionLevel::Q => 0x03,
            ErrorCorrectionLevel::H => 0x02,
        }
    }
}
