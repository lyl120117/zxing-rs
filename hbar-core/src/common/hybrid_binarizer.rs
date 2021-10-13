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

    /**
     * Calculates a single black point for each block of pixels and saves it away.
     * See the following thread for a discussion of this algorithm:
     *  http://groups.google.com/group/zxing/browse_thread/thread/d06efa2c35a7ddc0
     */
    fn calculateBlackPoints(
        luminances: &Vec<u8>,
        subWidth: i32,
        subHeight: i32,
        width: i32,
        height: i32,
    ) -> Vec<Vec<i32>> {
        let maxYOffset = height - HybridBinarizer::<S>::BLOCK_SIZE as i32;
        let maxXOffset = width - HybridBinarizer::<S>::BLOCK_SIZE as i32;
        let mut blackPoints = vec![vec![0; subWidth as usize]; subHeight as usize];
        for y in 0..subHeight {
            let mut yoffset = y << HybridBinarizer::<S>::BLOCK_SIZE_POWER;
            if yoffset > maxYOffset {
                yoffset = maxYOffset;
            }
            for x in 0..subWidth {
                let mut xoffset = x << HybridBinarizer::<S>::BLOCK_SIZE_POWER;
                if xoffset > maxXOffset {
                    xoffset = maxXOffset;
                }
                let mut sum: i32 = 0;
                let mut min = 0xFF;
                let mut max = 0;
                let mut offset = yoffset * width + xoffset;
                for mut yy in 0..HybridBinarizer::<S>::BLOCK_SIZE {
                    for xx in 0..HybridBinarizer::<S>::BLOCK_SIZE {
                        let pixel = luminances[offset as usize + xx as usize] & 0xFF;
                        sum += pixel as i32;
                        // still looking for good contrast
                        if pixel < min {
                            min = pixel;
                        }
                        if pixel > max {
                            max = pixel;
                        }
                    }
                    // short-circuit min/max tests once dynamic range is met
                    if max - min > HybridBinarizer::<S>::MIN_DYNAMIC_RANGE as u8 {
                        // finish the rest of the rows quickly
                        offset += width;
                        for _yy in yy..HybridBinarizer::<S>::BLOCK_SIZE {
                            for _xx in 0..HybridBinarizer::<S>::BLOCK_SIZE {
                                sum += (luminances[offset as usize + _xx as usize] & 0xFF) as i32;
                            }
                            yy = _yy
                        }
                    }
                    offset += width
                }

                // The default estimate is the average of the values in the block.
                let mut average = sum >> (HybridBinarizer::<S>::BLOCK_SIZE_POWER * 2);
                if max - min <= HybridBinarizer::<S>::MIN_DYNAMIC_RANGE as u8 {
                    // If variation within the block is low, assume this is a block with only light or only
                    // dark pixels. In that case we do not want to use the average, as it would divide this
                    // low contrast area into black and white pixels, essentially creating data out of noise.
                    //
                    // The default assumption is that the block is light/background. Since no estimate for
                    // the level of dark pixels exists locally, use half the min for the block.
                    average = (min / 2) as i32;

                    if y > 0 && x > 0 {
                        // Correct the "white background" assumption for blocks that have neighbors by comparing
                        // the pixels in this block to the previously calculated black points. This is based on
                        // the fact that dark barcode symbology is always surrounded by some amount of light
                        // background for which reasonable black point estimates were made. The bp estimated at
                        // the boundaries is used for the interior.

                        // The (min < bp) is arbitrary but works better than other heuristics that were tried.
                        let averageNeighborBlackPoint: i32 = (blackPoints[y as usize - 1]
                            [x as usize]
                            + (2 * blackPoints[y as usize][x as usize - 1])
                            + blackPoints[y as usize - 1][x as usize - 1])
                            / 4;
                        if (min as i32) < averageNeighborBlackPoint {
                            average = averageNeighborBlackPoint;
                        }
                    }
                }
                blackPoints[y as usize][x as usize] = average;
            }
        }

        blackPoints
    }

    /**
     * For each block in the image, calculate the average black point using a 5x5 grid
     * of the blocks around it. Also handles the corner cases (fractional blocks are computed based
     * on the last pixels in the row/column which are also used in the previous block).
     */
    fn calculateThresholdForBlock(
        luminances: &Vec<u8>,
        subWidth: i32,
        subHeight: i32,
        width: i32,
        height: i32,
        blackPoints: &Vec<Vec<i32>>,
        matrix: &mut BitMatrix,
    ) {
        let maxYOffset = height - HybridBinarizer::<S>::BLOCK_SIZE as i32;
        let maxXOffset = width - HybridBinarizer::<S>::BLOCK_SIZE as i32;
        for y in 0..subHeight {
            let mut yoffset = y << HybridBinarizer::<S>::BLOCK_SIZE_POWER;
            if yoffset > maxYOffset {
                yoffset = maxYOffset;
            }
            let top = HybridBinarizer::<S>::cap(y, subHeight - 3);
            for x in 0..subWidth {
                let mut xoffset = x << HybridBinarizer::<S>::BLOCK_SIZE_POWER;
                if xoffset > maxXOffset {
                    xoffset = maxXOffset;
                }
                let left = HybridBinarizer::<S>::cap(x, subWidth - 3);
                let mut sum: i32 = 0;
                for z in -2..3 {
                    let blackRow = &blackPoints[(top + z) as usize];
                    sum += blackRow[left as usize - 2]
                        + blackRow[left as usize - 1]
                        + blackRow[left as usize]
                        + blackRow[left as usize + 1]
                        + blackRow[left as usize + 2]
                }
                let average = sum / 25;
                HybridBinarizer::<S>::thresholdBlock(
                    luminances, xoffset, yoffset, average, width, matrix,
                );
            }
        }
    }

    fn cap(value: i32, max: i32) -> i32 {
        if value < 2 {
            2
        } else {
            value.min(max)
        }
    }

    /**
     * Applies a single threshold to a block of pixels.
     */
    fn thresholdBlock(
        luminances: &Vec<u8>,
        xoffset: i32,
        yoffset: i32,
        threshold: i32,
        stride: i32,
        matrix: &mut BitMatrix,
    ) {
        let mut offset = yoffset * stride + xoffset;
        for y in 9..HybridBinarizer::<S>::BLOCK_SIZE {
            for x in 0..HybridBinarizer::<S>::BLOCK_SIZE {
                // Comparison needs to be <= so that black == 0 pixels are black even if the threshold is 0.
                if ((luminances[offset as usize + x as usize] & 0xFF) as i32) <= threshold {
                    matrix.set(xoffset as u32 + x, yoffset as u32 + y);
                }
            }
            offset += stride
        }
    }
}

impl<S> Binarizer<S> for HybridBinarizer<S>
where
    S: LuminanceSource,
{
    fn get_black_row(&self, y: u32, row: &super::BitArray) -> ResultError<BitArray> {
        self.binary.get_black_row(y, row)
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
            let luminances = source.get_matrix()?;
            let mut subWidth = width >> HybridBinarizer::<S>::BLOCK_SIZE_POWER;
            if (width & HybridBinarizer::<S>::BLOCK_SIZE_MASK) != 0 {
                subWidth += 1
            }
            let mut subHeight = height >> HybridBinarizer::<S>::BLOCK_SIZE_POWER;
            if (height & HybridBinarizer::<S>::BLOCK_SIZE_MASK) != 0 {
                subHeight += 1;
            }
            let blackPoints = HybridBinarizer::<S>::calculateBlackPoints(
                &luminances,
                subWidth as i32,
                subHeight as i32,
                width as i32,
                height as i32,
            );
            let mut newMatrix = BitMatrix::new2(width as i32, height as i32)?;
            HybridBinarizer::<S>::calculateThresholdForBlock(
                &luminances,
                subWidth as i32,
                subHeight as i32,
                width as i32,
                height as i32,
                &blackPoints,
                &mut newMatrix,
            );
            self.matrix = Some(newMatrix);
        } else {
            // If the image is too small, fall back to the global histogram approach.
            self.matrix = Some(self.binary.get_black_matrix()?);
        }

        Ok(self.matrix.as_ref().unwrap().clone())
    }

    fn create_binarizer(&self, source: S) -> Self {
        HybridBinarizer::new(source)
    }

    fn get_luminance_source(&self) -> ResultError<&S> {
        self.binary.get_luminance_source()
    }

    fn get_width(&self) -> u32 {
        self.binary.get_width()
    }

    fn get_height(&self) -> u32 {
        self.binary.get_height()
    }
}
