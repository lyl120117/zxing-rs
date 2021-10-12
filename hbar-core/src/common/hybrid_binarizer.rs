use crate::common::GlobalHistogramBinarizer;
use crate::common::{BitArray, BitMatrix};
use crate::{Binarizer, LuminanceSource, ResultError};

// use std::cell::RefCell;

pub struct HybridBinarizer<S> {
    matrix: Option<BitMatrix>,
    binary: GlobalHistogramBinarizer<S>,
}

impl<S> HybridBinarizer<S> {
    // This class uses 5x5 blocks to compute local luminance, where each block is 8x8 pixels.
    // So this is the smallest dimension in each axis we can accept.
    const BLOCK_SIZE_POWER: i32 = 3;
    const BLOCK_SIZE: u32 = 1 << HybridBinarizer::<S>::BLOCK_SIZE_POWER; // ...0100...00
    const BLOCK_SIZE_MASK: u32 = HybridBinarizer::<S>::BLOCK_SIZE - 1; // ...0011...11
    const MINIMUM_DIMENSION: u32 = HybridBinarizer::<S>::BLOCK_SIZE * 5;
    const MIN_DYNAMIC_RANGE: u32 = 24;

    pub fn new(source: S) -> HybridBinarizer<S> {
        let binary = GlobalHistogramBinarizer::new(source);
        HybridBinarizer {
            binary: binary,
            matrix: None,
        }
    }
}

impl<S> Binarizer<S> for HybridBinarizer<S>
where
    S: LuminanceSource,
{
    fn get_black_row(&self, y: u32, row: &super::BitArray) -> ResultError<BitArray> {
        todo!()
    }

    /**
     * Calculates the final BitMatrix once for all requests. This could be called once from the
     * constructor instead, but there are some advantages to doing it lazily, such as making
     * profiling easier, and not doing heavy lifting when callers don't expect it.
     */
    fn get_black_matrix(&mut self) -> ResultError<BitMatrix> {
        if let Some(matrix) = &self.matrix {
            return Ok((*matrix).clone());
        }
        let source = self.get_luminance_source()?;
        let width = source.get_width();
        let height = source.get_height();
        if width >= HybridBinarizer::<S>::MINIMUM_DIMENSION
            && height >= HybridBinarizer::<S>::MINIMUM_DIMENSION
        {
        } else {
            self.matrix = Some(self.binary.get_black_matrix()?);
        }

        todo!()
    }

    fn create_binarizer(&self, source: S) -> Self {
        todo!()
    }

    fn get_luminance_source(&self) -> ResultError<&S> {
        todo!()
    }

    fn get_width(&self) -> u32 {
        todo!()
    }

    fn get_height(&self) -> u32 {
        todo!()
    }
}
