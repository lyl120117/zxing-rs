use crate::common::{BitArray, BitMatrix};
use crate::{Binarizer, LuminanceSource};

pub struct GlobalHistogramBinarizer {
    luminances: Vec<u8>,
    buckets: Vec<i32>,
    source: Box<dyn LuminanceSource>,
}

impl GlobalHistogramBinarizer {
    const LUMINANCE_BITS: i32 = 5;
    const LUMINANCE_SHIFT: i32 = 8 - GlobalHistogramBinarizer::LUMINANCE_BITS;
    const LUMINANCE_BUCKETS: i32 = 1 << GlobalHistogramBinarizer::LUMINANCE_BITS;

    pub fn new(source: Box<dyn LuminanceSource>) -> GlobalHistogramBinarizer {
        GlobalHistogramBinarizer {
            source: source,
            luminances: vec![0],
            buckets: vec![0; GlobalHistogramBinarizer::LUMINANCE_BUCKETS as usize],
        }
    }
}

impl Binarizer for GlobalHistogramBinarizer {
    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    fn get_black_row(&self, y: u32, _row: &BitArray) -> Result<BitArray, crate::Error> {
        let source = self.get_luminance_source();
        let width = source.get_width();
        let mut row = _row;
        if (row.get_size() as u32) < width {
            row = &mut BitArray::new1(width);
        } else {
            // row.clear()
        }
        todo!()
    }

    fn get_black_matrix(&self) -> Result<BitMatrix, crate::Error> {
        todo!()
    }

    fn create_binarizer(
        &self,
        source: Box<dyn LuminanceSource>,
    ) -> Result<Box<dyn Binarizer>, crate::Error> {
        todo!()
    }

    fn get_width(&self) -> u32 {
        todo!()
    }

    fn get_height(&self) -> u32 {
        todo!()
    }

    fn get_luminance_source(&self) -> &Box<dyn LuminanceSource> {
        &self.source
    }
}
