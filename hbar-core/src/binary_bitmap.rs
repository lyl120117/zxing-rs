use crate::common::BitMatrix;
use crate::Binarizer;
use crate::{Error, ResultError};

pub struct BinaryBitmap<B, S> {
    binarizer: B,
    tmp: Option<S>,
    matrix: Option<BitMatrix>,
}

impl<B, S> BinaryBitmap<B, S>
where
    B: Binarizer<S>,
{
    pub fn new(binarizer: B) -> BinaryBitmap<B, S> {
        BinaryBitmap {
            binarizer: binarizer,
            matrix: None,
            tmp: None,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.binarizer.get_width()
    }
}
