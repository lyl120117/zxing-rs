use crate::common::BitMatrix;
use crate::common::DecoderResult;
use crate::qrcode::Decoder;
use crate::Reader;
use crate::ResultPoint;
use crate::Results;
use crate::{Binarizer, BinaryBitmap, LuminanceSource};
use crate::{DecodeHintType, DecodeHintValue};
use crate::{Error, ResultError};

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
        let leftTopBlack = match image.getTopLeftOnBit() {
            Some(v) => v,
            None => {
                return Err(Error::NotFoundException(String::from(
                    "Error get left top black.",
                )))
            }
        };
        let rightBottomBlack = match image.getBottomRightOnBit() {
            Some(v) => v,
            None => {
                return Err(Error::NotFoundException(String::from(
                    "Error get right bottom black.",
                )))
            }
        };

        let moduleSize = QRCodeReader::moduleSize(&leftTopBlack, image)?;

        let mut top = leftTopBlack[1];
        let bottom = rightBottomBlack[1];
        let mut left = leftTopBlack[0];
        let mut right = rightBottomBlack[0];

        // Sanity check!
        if left >= right || top >= bottom {
            return Err(Error::NotFoundException(String::from(
                "Sanity check failed",
            )));
        }
        if bottom - top != right - left {
            // Special case, where bottom-right module wasn't black so we found something else in the last row
            // Assume it's a square, so use height as the width
            right = left + (bottom - top);
            if right >= image.getWidth() {
                // Abort if that would not make sense -- off image
                return Err(Error::NotFoundException(String::from(
                    "Abort if that would not make sense -- off image",
                )));
            }
        }

        let matrixWidth = ((right - left + 1) as f32 / moduleSize).round() as i32;
        let matrixHeight = ((bottom - top + 1) as f32 / moduleSize).round() as i32;
        if matrixWidth <= 0 || matrixHeight <= 0 {
            return Err(Error::NotFoundException(String::from("")));
        }
        if matrixHeight != matrixWidth {
            // Only possibly decode square regions
            return Err(Error::NotFoundException(String::from(
                "Only possibly decode square regions",
            )));
        }

        // Push in the "border" by half the module width so that we start
        // sampling in the middle of the module. Just in case the image is a
        // little off, this will help recover.
        let nudge = (moduleSize / 2.0f32) as i32;
        top += nudge;
        left += nudge;

        // But careful that this does not sample off the edge
        // "right" is the farthest-right valid pixel location -- right+1 is not necessarily
        // This is positive by how much the inner x loop below would be too large
        let nudgedTooFarRight = left + ((matrixWidth - 1) as f32 * moduleSize) as i32 - right;
        if nudgedTooFarRight > 0 {
            if nudgedTooFarRight > nudge {
                // Neither way fits; abort
                return Err(Error::NotFoundException(String::from("")));
            }
            left -= nudgedTooFarRight;
        }
        // See logic above
        let nudgedTooFarDown = top + ((matrixHeight - 1) as f32 * moduleSize) as i32 - bottom;
        if nudgedTooFarDown > 0 {
            if nudgedTooFarDown > nudge {
                // Neither way fits; abort
                return Err(Error::NotFoundException(String::from("")));
            }
            top -= nudgedTooFarDown;
        }

        // Now just read off the bits
        let mut bits = BitMatrix::new2(matrixWidth, matrixHeight)?;
        for y in 0..matrixHeight {
            let iOffset = top + (y as f32 * moduleSize) as i32;
            for x in 0..matrixWidth {
                if image.get(
                    (left + (x as f32 * moduleSize) as i32) as u32,
                    iOffset as u32,
                ) {
                    bits.set(x as u32, y as u32);
                }
            }
        }

        Ok(bits)
    }

    fn moduleSize(leftTopBlack: &Vec<i32>, image: &BitMatrix) -> ResultError<f32> {
        let height = image.getHeight();
        let width = image.getWidth();
        let mut x = leftTopBlack[0];
        let mut y = leftTopBlack[1];
        let mut inBlack = true;
        let mut transitions = 0;
        while x < width && y < height {
            if inBlack != image.get(x as u32, y as u32) {
                transitions += 1;
                if transitions == 5 {
                    break;
                }
                inBlack = !inBlack;
            }
            x += 1;
            y += 1;
        }
        if x == width || y == height {
            return Err(Error::NotFoundException(String::from(
                "Not found module size.",
            )));
        }

        Ok((x - leftTopBlack[0]) as f32 / 7.0f32)
    }
}

impl<B, S> Reader<B, S> for QRCodeReader
where
    B: Binarizer<S>,
    S: LuminanceSource,
{
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
            println!("QRCodeReader decode_hints PureBarcode");
            let bits = QRCodeReader::extractPureBits(&image.getBlackMatrix()?);
            // decoderResult = self.decoder.decode(&bits)
        } else {
            todo!()
        }

        todo!()
    }

    fn reset(&self) {
        todo!()
    }
}
