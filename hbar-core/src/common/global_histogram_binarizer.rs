use crate::common::{BitArray, BitMatrix};
use crate::{Binarizer, Error, LuminanceSource, ResultError};

use std::cell::RefCell;

pub struct GlobalHistogramBinarizer<S> {
    luminances: RefCell<Vec<u8>>,
    buckets: RefCell<Vec<i32>>,
    source: S,
}

impl<S> GlobalHistogramBinarizer<S> {
    const LUMINANCE_BITS: i32 = 5;
    const LUMINANCE_SHIFT: i32 = 8 - GlobalHistogramBinarizer::<S>::LUMINANCE_BITS;
    const LUMINANCE_BUCKETS: i32 = 1 << GlobalHistogramBinarizer::<S>::LUMINANCE_BITS;

    pub fn new(source: S) -> GlobalHistogramBinarizer<S> {
        GlobalHistogramBinarizer {
            source: source,
            luminances: RefCell::new(vec![0]),
            buckets: RefCell::new(vec![
                0;
                GlobalHistogramBinarizer::<S>::LUMINANCE_BUCKETS
                    as usize
            ]),
        }
    }

    fn init_arrays(&self, luminance_size: i32) {
        let luminance_size = luminance_size as usize;
        if self.luminances.borrow().len() == luminance_size {
            for x in 0..self.luminances.borrow().len() {
                self.luminances.borrow_mut()[x] = 0;
            }
        }
        for x in 0..GlobalHistogramBinarizer::<S>::LUMINANCE_BUCKETS {
            self.buckets.borrow_mut()[x as usize] = 0;
        }
    }
}

impl<S> Binarizer<S> for GlobalHistogramBinarizer<S>
where
    S: LuminanceSource,
{
    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    fn get_black_row(&self, y: u32, _row: &BitArray) -> ResultError<BitArray> {
        let source = self.get_luminance_source()?;
        let width = source.get_width();
        let mut row = (*_row).clone();
        if (row.get_size() as u32) < width {
            row = BitArray::new1(width);
        } else {
            row.clear()
        }

        self.init_arrays(width as i32);
        let localLuminances = source.get_row(y as i32, &(*self.luminances.borrow()))?;
        todo!()
    }

    fn get_black_matrix(&mut self) -> ResultError<BitMatrix> {
        todo!()
    }

    fn create_binarizer(&self, source: S) -> Self {
        GlobalHistogramBinarizer::new(source)
    }

    fn get_width(&self) -> u32 {
        todo!()
    }

    fn get_height(&self) -> u32 {
        todo!()
    }

    fn get_luminance_source(&self) -> ResultError<&S> {
        Ok(&self.source)
    }
}
