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
    fn decode(&self, image: &crate::BinaryBitmap<B, S>) -> crate::ResultError<crate::Results> {
        todo!()
    }

    fn decode_hints(
        &self,
        image: &crate::BinaryBitmap<B, S>,
        hints: &HashMap<crate::DecodeHintType, crate::DecodeHintValue>,
    ) -> crate::ResultError<crate::Results> {
        todo!()
    }

    fn reset(&self) {
        todo!()
    }
}
