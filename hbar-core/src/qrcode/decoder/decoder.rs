use crate::common::BitMatrix;
use crate::common::DecoderResult;
use crate::common::{GenericGFEnum, ReedSolomonDecoder};
use crate::ResultError;
use crate::{DecodeHintType, DecodeHintValue};

use std::collections::HashMap;
use std::rc::Rc;
/**
 * <p>The main class which implements QR Code decoding -- as opposed to locating and extracting
 * the QR Code from an image.</p>
 *
 */
pub struct Decoder {
    rsDecoder: ReedSolomonDecoder,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            rsDecoder: ReedSolomonDecoder::new(Rc::new(GenericGFEnum::QrCodeField256.get())),
        }
    }

    pub fn decode_image(&self, image: &Vec<Vec<bool>>) -> ResultError<DecoderResult> {
        let hints = HashMap::new();
        self.decode_image_hints(image, &hints)
    }

    /**
     * <p>Convenience method that can decode a QR Code represented as a 2D array of booleans.
     * "true" is taken to mean a black module.</p>
     *
     * @param image booleans representing white/black QR Code modules
     * @param hints decoding hints that should be used to influence decoding
     * @return text and bytes encoded within the QR Code
     * @throws FormatException if the QR Code cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    pub fn decode_image_hints(
        &self,
        image: &Vec<Vec<bool>>,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> ResultError<DecoderResult> {
        self.decode_hints(&BitMatrix::parse_image(image)?, hints)
    }

    pub fn decode(&self, bits: &BitMatrix) -> ResultError<DecoderResult> {
        let hints = HashMap::new();
        self.decode_hints(bits, &hints)
    }

    /**
     * <p>Decodes a QR Code represented as a {@link BitMatrix}. A 1 or "true" is taken to mean a black module.</p>
     *
     * @param bits booleans representing white/black QR Code modules
     * @param hints decoding hints that should be used to influence decoding
     * @return text and bytes encoded within the QR Code
     * @throws FormatException if the QR Code cannot be decoded
     * @throws ChecksumException if error correction fails
     */

    pub fn decode_hints(
        &self,
        bits: &BitMatrix,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> ResultError<DecoderResult> {
        // Construct a parser and read version, error-correction level
        todo!()
    }
}
