use crate::BarcodeFormat;
use strum_macros::EnumString;
use strum_macros::ToString;
/**
 * Encapsulates a type of hint that a caller may pass to a barcode reader to help it
 * more quickly or accurately decode it. It is up to implementations to decide what,
 * if anything, to do with the information that is supplied.
 */

#[derive(Debug, PartialEq, Eq, Hash, EnumString, ToString)]
pub enum DecodeHintType {
    /**
     * Unspecified, application-specific hint. Maps to an unspecified {@link Object}.
     */
    OTHER,

    /**
     * Image is a pure monochrome image of a barcode. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    PureBarcode,

    /**
     * Image is known to be of one of a few possible formats.
     * Maps to a {@link List} of {@link BarcodeFormat}s.
     */
    PossibleFormats,

    /**
     * Spend more time to try to find a barcode; optimize for accuracy, not speed.
     * Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    TryHarder,

    /**
     * Specifies what character encoding to use when decoding, where applicable (type String)
     */
    CharacterSet,

    /**
     * Allowed lengths of encoded data -- reject anything else. Maps to an {@code int[]}.
     */
    AllowedLengths,

    /**
     * Assume Code 39 codes employ a check digit. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    AssumeCode39CheckDigit,

    /**
     * Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed.
     * For example this affects FNC1 handling for Code 128 (aka GS1-128). Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    AssumeGs1,

    /**
     * If true, return the start and end digits in a Codabar barcode instead of stripping them. They
     * are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them
     * to not be. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    ReturnCodabarStartEnd,

    /**
     * The caller needs to be notified via callback when a possible {@link ResultPoint}
     * is found. Maps to a {@link ResultPointCallback}.
     */
    NeedResultPointCallback,

    /**
     * Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this.
     * Maps to an {@code int[]} of the allowed extension lengths, for example [2], [5], or [2, 5].
     * If it is optional to have an extension, do not set this hint. If this is set,
     * and a UPC or EAN barcode is found but an extension is not, then no result will be returned
     * at all.
     */
    AllowedEanExtensions,

    /**
     * If true, also tries to decode as inverted image. All configured decoders are simply called a
     * second time with an inverted image. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    AlsoInverted,
}

pub enum DecodeHintValue {
    BOOL(bool),
    VecI32(Vec<i32>),
    VecBarcodeFormat(Vec<BarcodeFormat>),
}

impl DecodeHintValue {
    pub fn get_bool(&self) -> bool {
        match self {
            DecodeHintValue::BOOL(v) => *v,
            _ => panic!(),
        }
    }

    pub fn get_vec_i32(&self) -> &Vec<i32> {
        match self {
            DecodeHintValue::VecI32(v) => v,
            _ => panic!(),
        }
    }

    pub fn get_vec_barcode_format(&self) -> &Vec<BarcodeFormat> {
        match self {
            DecodeHintValue::VecBarcodeFormat(v) => v,
            _ => panic!(),
        }
    }
}
