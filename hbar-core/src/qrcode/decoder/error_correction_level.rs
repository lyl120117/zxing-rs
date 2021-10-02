use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString)]
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
