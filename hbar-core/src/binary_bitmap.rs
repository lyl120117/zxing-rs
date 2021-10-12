use crate::common::{BitArray, BitMatrix};
use crate::{Binarizer, LuminanceSource};
use crate::{Error, ResultError};

use std::cell::RefCell;

pub struct BinaryBitmap<B, S> {
    binarizer: RefCell<B>,
    _tmp: Option<S>,
    matrix: RefCell<BitMatrix>,
    matrix_inited: bool,
}

impl<B, S> BinaryBitmap<B, S>
where
    B: Binarizer<S>,
    S: LuminanceSource,
{
    pub fn new(binarizer: B) -> BinaryBitmap<B, S> {
        BinaryBitmap {
            binarizer: RefCell::new(binarizer),
            matrix: RefCell::new(BitMatrix::new2(1, 1)),
            _tmp: None,
            matrix_inited: false,
        }
    }

    /**
     * @return The width of the bitmap.
     */
    pub fn get_width(&self) -> u32 {
        self.binarizer.borrow().get_width()
    }

    /**
     * @return The height of the bitmap.
     */
    pub fn get_height(&self) -> u32 {
        self.binarizer.borrow().get_height()
    }

    /**
     * Converts one row of luminance data to 1 bit data. May actually do the conversion, or return
     * cached data. Callers should assume this method is expensive and call it as seldom as possible.
     * This method is intended for decoding 1D barcodes and may choose to apply sharpening.
     *
     * @param y The row to fetch, which must be in [0, bitmap height)
     * @param row An optional preallocated array. If null or too small, it will be ignored.
     *            If used, the Binarizer will call BitArray.clear(). Always use the returned object.
     * @return The array of bits for this row (true means black).
     * @throws NotFoundException if row can't be binarized
     */
    pub fn get_black_row(&self, y: u32, row: &BitArray) -> ResultError<BitArray> {
        self.binarizer.borrow().get_black_row(y, row)
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    pub fn getBlackMatrix(&self) -> ResultError<BitMatrix> {
        // The matrix is created on demand the first time it is requested, then cached. There are two
        // reasons for this:
        // 1. This work will never be done if the caller only installs 1D Reader objects, or if a
        //    1D Reader finds a barcode before the 2D Readers run.
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        if !self.matrix_inited {
            *self.matrix.borrow_mut() = self.binarizer.borrow_mut().get_black_matrix()?;
        }
        Ok(self.matrix.borrow().clone())
    }

    /**
     * @return Whether this bitmap can be cropped.
     */
    pub fn is_crop_supported(&self) -> ResultError<bool> {
        Ok(self
            .binarizer
            .borrow()
            .get_luminance_source()?
            .is_crop_supported())
    }

    /**
     * Returns a new object with cropped image data. Implementations may keep a reference to the
     * original data rather than a copy. Only callable if isCropSupported() is true.
     *
     * @param left The left coordinate, which must be in [0,getWidth())
     * @param top The top coordinate, which must be in [0,getHeight())
     * @param width The width of the rectangle to crop.
     * @param height The height of the rectangle to crop.
     * @return A cropped version of this object.
     */
    pub fn crop(
        &self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> ResultError<BinaryBitmap<B, S>> {
        let newSource = self
            .binarizer
            .borrow()
            .get_luminance_source()?
            .crop(left, top, width, height)?;
        // Ok(BinaryBitmap::new(self.binarizer.borrow().create_binarizer(*newSource)))
        todo!()
    }
}
