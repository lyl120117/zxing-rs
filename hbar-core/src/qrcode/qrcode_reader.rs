use crate::qrcode::Decoder;
use crate::Reader;
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
    fn decode(&self, image: &crate::BinaryBitmap<B, S>) -> crate::ResultError<crate::Results> {
        let hints: HashMap<crate::DecodeHintType, crate::DecodeHintValue> = HashMap::new();
        self.decode_hints(image, &hints)
    }

    fn decode_hints(
        &self,
        image: &crate::BinaryBitmap<B, S>,
        hints: &HashMap<crate::DecodeHintType, crate::DecodeHintValue>,
    ) -> crate::ResultError<crate::Results> {
        println!("QRCodeReader decode_hints hints: {:?}", hints);
        todo!()
    }

    fn reset(&self) {
        todo!()
    }
}
