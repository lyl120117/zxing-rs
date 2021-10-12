use crate::BinaryBitmap;
use crate::QRCodeReader;
use crate::Reader;
use crate::Results;
use crate::{BarcodeFormat, DecodeHintType, DecodeHintValue};
use crate::{Error, ResultError};

use std::cell::RefCell;
use std::collections::HashMap;

/**
 * MultiFormatReader is a convenience class and the main entry point into the library for most uses.
 * By default it attempts to decode all barcode formats that the library supports. Optionally, you
 * can provide a hints object to request different behavior, for example only decoding QR codes.
 *
 */
pub struct MultiFormatReader<B, S> {
    hints: RefCell<HashMap<DecodeHintType, DecodeHintValue>>,
    readers: RefCell<Vec<Box<dyn Reader<B, S>>>>,
}

impl<B, S> Reader<B, S> for MultiFormatReader<B, S> {
    fn decode(&self, image: &BinaryBitmap<B, S>) -> ResultError<Results> {
        let hint: HashMap<DecodeHintType, DecodeHintValue> = HashMap::new();
        self.set_hints(hint);

        self.decode_internal(image)
    }

    fn decode_hints(
        &self,
        image: &BinaryBitmap<B, S>,
        hints: &HashMap<DecodeHintType, DecodeHintValue>,
    ) -> ResultError<Results> {
        todo!()
    }

    fn reset(&self) {
        todo!()
    }
}

impl<B, S> MultiFormatReader<B, S> {
    pub fn new() -> ResultError<MultiFormatReader<B, S>> {
        Ok(MultiFormatReader {
            hints: RefCell::new(HashMap::new()),
            readers: RefCell::new(Vec::new()),
        })
    }

    /**
     * This method adds state to the MultiFormatReader. By setting the hints once, subsequent calls
     * to decodeWithState(image) can reuse the same set of readers without reallocating memory. This
     * is important for performance in continuous scan clients.
     *
     * @param hints The set of hints to use for subsequent calls to decode(image)
     */
    pub fn set_hints(&self, hints: HashMap<DecodeHintType, DecodeHintValue>) {
        self.hints.borrow_mut().clear();
        for (k, v) in hints.clone() {
            self.hints.borrow_mut().insert(k, v);
        }
        let hints = &hints;
        // for (k, v) in &hints.into_keys().zip(&hints.into_values()) {
        //     self.hints.borrow_mut().insert(k, v);
        // }
        let try_harder = hints.contains_key(&DecodeHintType::TryHarder);
        let formats = hints.get(&DecodeHintType::PossibleFormats);
        // .unwrap()
        // .get_vec_barcode_format();
        let mut readers: Vec<Box<dyn Reader<B, S>>> = Vec::new();
        if let Some(formats) = formats {
            let formats = formats.get_vec_barcode_format();
            let add_one_dreader = formats.contains(&BarcodeFormat::UpcA)
                || formats.contains(&BarcodeFormat::UpcE)
                || formats.contains(&BarcodeFormat::Ean13)
                || formats.contains(&BarcodeFormat::Ean8)
                || formats.contains(&BarcodeFormat::CodeBar)
                || formats.contains(&BarcodeFormat::Code93)
                || formats.contains(&BarcodeFormat::Code39)
                || formats.contains(&BarcodeFormat::Code128)
                || formats.contains(&BarcodeFormat::ITF)
                || formats.contains(&BarcodeFormat::RSS14)
                || formats.contains(&BarcodeFormat::RssExpanded);
            // Put 1D readers upfront in "normal" mode
            if add_one_dreader && !try_harder {
                // readers.push(value)
                todo!()
            }
            if formats.contains(&BarcodeFormat::QRCode) {
                readers.push(Box::new(QRCodeReader::new()))
            }

            if formats.contains(&BarcodeFormat::DataMatrix) {
                todo!()
            }

            if formats.contains(&BarcodeFormat::Aztec) {
                todo!()
            }

            if formats.contains(&BarcodeFormat::PDF417) {
                todo!()
            }

            if formats.contains(&BarcodeFormat::MaxiCode) {
                todo!()
            }
            // At end in "try harder" mode
            if add_one_dreader && try_harder {
                todo!()
            }
            if readers.is_empty() {
                if !try_harder {
                    todo!()
                }
                readers.push(Box::new(QRCodeReader::new()));

                if try_harder {
                    todo!()
                }
            }
        }
        self.readers.borrow_mut().clear();
        for reader in readers {
            self.readers.borrow_mut().push(reader)
        }
    }

    fn decode_internal(&self, image: &BinaryBitmap<B, S>) -> ResultError<Results> {
        if self.readers.borrow().is_empty() {
            return Err(Error::NotFoundException(String::from(
                "Not found any readers!",
            )));
        }
        for reader in self.readers.borrow().iter() {
            let result = reader.decode_hints(image, &(*self.hints.borrow()))?;
            return Ok(result);
        }
        Err(Error::NotFoundException(String::from("Cannot decode...")))
    }
}
