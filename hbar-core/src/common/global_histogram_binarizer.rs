use crate::common::{BitArray, BitMatrix};
use crate::{Binarizer, Error, LuminanceSource, ResultError};

use std::cell::RefCell;

/**
 * This Binarizer implementation uses the old ZXing global histogram approach. It is suitable
 * for low-end mobile devices which don't have enough CPU or memory to use a local thresholding
 * algorithm. However, because it picks a global black point, it cannot handle difficult shadows
 * and gradients.
 *
 * Faster mobile devices and all desktop applications should probably use HybridBinarizer instead.
 *
 */
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

    fn estimateBlackPoint(buckets: &Vec<i32>) -> ResultError<i32> {
        // Find the tallest peak in the histogram.
        let numBuckets = buckets.len();
        let mut maxBucketCount = 0;
        let mut firstPeak = 0;
        let mut firstPeakSize = 0;
        for x in 0..numBuckets {
            if buckets[x] > firstPeakSize {
                firstPeak = x;
                firstPeakSize = buckets[x];
            }
            if buckets[x] > maxBucketCount {
                maxBucketCount = buckets[x];
            }
        }

        // Find the second-tallest peak which is somewhat far from the tallest peak.
        let mut secondPeak = 0;
        let mut secondPeakScore = 0;
        for x in 0..numBuckets {
            let distanceToBiggest = (x - firstPeak) as i32;
            // Encourage more distant second peaks by multiplying by square of distance.
            let score = buckets[x] * distanceToBiggest * distanceToBiggest;
            if score > secondPeakScore {
                secondPeak = x;
                secondPeakScore = score;
            }
        }

        // Make sure firstPeak corresponds to the black peak.
        if firstPeak > secondPeak {
            let temp = firstPeak;
            firstPeak = secondPeak;
            secondPeak = temp;
        }

        // If there is too little contrast in the image to pick a meaningful black point, throw rather
        // than waste time trying to decode the image, and risk false positives.
        if secondPeak - firstPeak <= numBuckets / 16 {
            return Err(Error::NotFoundException(String::from(
                "Too little contrast in the image",
            )));
        }

        // Find a valley between them that is low and closer to the white peak.
        let mut bestValley = secondPeak - 1;
        let mut bestValleyScore = -1;
        for x in (firstPeak + 1..secondPeak).rev() {
            let fromFirst = (x - firstPeak) as i32;
            let score =
                fromFirst * fromFirst * (secondPeak - x) as i32 * (maxBucketCount - buckets[x]);
            if score > bestValleyScore {
                bestValley = x;
                bestValleyScore = score;
            }
        }

        Ok((bestValley as i32) << GlobalHistogramBinarizer::<S>::LUMINANCE_SHIFT)
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
        for x in 0..width {
            let x = (localLuminances[x as usize] & 0xFF)
                >> GlobalHistogramBinarizer::<S>::LUMINANCE_SHIFT;
            self.buckets.borrow_mut()[x as usize] += 1;
        }
        let blackPoint =
            GlobalHistogramBinarizer::<S>::estimateBlackPoint(&(*self.buckets.borrow()))?;
        println!("blackPoint: {}", blackPoint);
        if width < 3 {
            // Special case for very small images
            for x in 0..width {
                if ((localLuminances[x as usize] & 0xff) as i32) < blackPoint {
                    row.set(x as i32)
                }
            }
        } else {
            let mut left = localLuminances[0] & 0xff;
            let mut center = localLuminances[1] & 0xff;
            for x in 1..(width - 1) {
                let right = localLuminances[x as usize + 1] & 0xff;
                // A simple -1 4 -1 box filter with a weight of 2.
                if (((center * 4) - left - right) as i32) / 2 < blackPoint {
                    row.set(x as i32)
                }
                left = center;
                center = right;
            }
        }
        Ok(row)
    }

    // Does not sharpen the data, as this call is intended to only be used by 2D Readers.
    fn get_black_matrix(&mut self) -> ResultError<BitMatrix> {
        let source = self.get_luminance_source()?;
        let width = source.get_width() as i32;
        let height = source.get_height() as i32;
        let mut matrix = BitMatrix::new2(width, height);

        // Quickly calculates the histogram by sampling four rows from the image. This proved to be
        // more robust on the blackbox tests than sampling a diagonal as we used to do.
        self.init_arrays(width);
        for y in 1..5 {
            let row = height * y / 5;
            let localLuminances = source.get_row(row, &(*self.luminances.borrow()))?;
            let right = (width * 4) / 5;
            for x in width / 5..right {
                let pixel = localLuminances[x as usize] & 0xFF;
                self.buckets.borrow_mut()
                    [(pixel >> GlobalHistogramBinarizer::<S>::LUMINANCE_SHIFT) as usize] += 1;
            }
        }
        let blackPoint =
            GlobalHistogramBinarizer::<S>::estimateBlackPoint(&(*self.buckets.borrow()))?;

        // We delay reading the entire image luminance until the black point estimation succeeds.
        // Although we end up reading four rows twice, it is consistent with our motto of
        // "fail quickly" which is necessary for continuous scanning.
        let localLuminances = source.get_matrix()?;
        for y in 0..height {
            let offset = y * width;
            for x in 0..width {
                let pixel = localLuminances[(offset + x) as usize] & 0xFF;
                if (pixel as i32) < blackPoint {
                    matrix.set(x as u32, y as u32)
                }
            }
        }

        Ok(matrix)
    }

    fn create_binarizer(&self, source: S) -> Self {
        GlobalHistogramBinarizer::new(source)
    }

    fn get_width(&self) -> u32 {
        self.source.get_width()
    }

    fn get_height(&self) -> u32 {
        self.source.get_height()
    }

    fn get_luminance_source(&self) -> ResultError<&S> {
        Ok(&self.source)
    }
}
