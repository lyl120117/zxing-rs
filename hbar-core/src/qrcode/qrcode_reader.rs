use crate::common::BitMatrix;
use crate::common::DecoderResult;
use crate::qrcode::Decoder;
use crate::BinaryBitmap;
use crate::Reader;
use crate::ResultError;
use crate::ResultPoint;
use crate::Results;
use crate::{DecodeHintType, DecodeHintValue};

use std::collections::HashMap;

/**
 * This implementation can detect and decode QR Codes in an image.
 *
 */
pub struct QRCodeReader {
    decoder: Decoder,
}

impl QRCodeReader {
    pub fn new() -> QRCodeReader {
        QRCodeReader {
            decoder: Decoder::new(),
        }
    }

    /**
     * This method detects a code in a "pure" image -- that is, pure monochrome image
     * which contains only an unrotated, unskewed, image of a code, with some white border
     * around it. This is a specialized method that works exceptionally fast in this special
     * case.
     */
    fn extractPureBits(image: &BitMatrix) -> ResultError<BitMatrix> {
        let leftTopBlack = image.getTopLeftOnBit();
        let rightBottomBlack = image.getBottomRightOnBit();

        todo!()
    }
}

impl<B, S> Reader<B, S> for QRCodeReader {
    /**
     * Locates and decodes a QR code in an image.
     *
     * @return a String representing the content encoded by the QR code
     * @throws NotFoundException if a QR code cannot be found
     * @throws FormatException if a QR code cannot be decoded
     * @throws ChecksumException if error correction fails
     */
    fn decode(&self, image: &BinaryBitmap<B, S>) -> ResultError<Results> {
        let hints: HashMap<DecodeHintType, DecodeHintValue> = HashMap::new();
        self.decode_hints(image, &hints)
    }

    fn decode_hints(
        &self,
        image: &BinaryBitmap<B, S>,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> ResultError<Results> {
        println!("QRCodeReader decode_hints hints: {:?}", hints);
        let decoderResult: DecoderResult;
        let mut points: Vec<ResultPoint>;
        if hints.contains_key(&DecodeHintType::PureBarcode) {
            println!("QRCodeReader decode_hints PureBarcode")
        } else {
            todo!()
        }

        todo!()
    }

    fn reset(&self) {
        todo!()
    }
}
