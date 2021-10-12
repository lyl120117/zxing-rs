use crate::Error;

use std::fmt;

pub trait LuminanceSource {
    /**
     * Fetches one row of luminance data from the underlying platform's bitmap. Values range from
     * 0 (black) to 255 (white). Because Java does not have an unsigned byte type, callers will have
     * to bitwise and with 0xff for each value. It is preferable for implementations of this method
     * to only fetch this row rather than the whole image, since no 2D Readers may be installed and
     * getMatrix() may never be called.
     *
     * @param y The row to fetch, which must be in [0,getHeight())
     * @param row An optional preallocated array. If null or too small, it will be ignored.
     *            Always use the returned object, and ignore the .length of the array.
     * @return An array containing the luminance data.
     */
    fn get_row(&self, y: i32, row: &Vec<u8>) -> Result<Vec<u8>, Error>;

    /**
     * Fetches luminance data for the underlying bitmap. Values should be fetched using:
     * {@code int luminance = array[y * width + x] & 0xff}
     *
     * @return A row-major 2D array of luminance values. Do not use result.length as it may be
     *         larger than width * height bytes on some platforms. Do not modify the contents
     *         of the result.
     */
    fn get_matrix(&self) -> Result<Vec<u8>, Error>;

    /**
     * @return The width of the bitmap.
     */
    fn get_width(&self) -> u32;

    /**
     * @return The height of the bitmap.
     */
    fn get_height(&self) -> u32;

    /**
     * @return Whether this subclass supports cropping.
     */
    fn is_crop_supported(&self) -> bool {
        false
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
    fn crop(
        &self,
        _left: u32,
        _top: u32,
        _width: u32,
        _height: u32,
    ) -> Result<Box<dyn LuminanceSource>, Error> {
        Err(Error::UnsupportedOperationException(String::from(
            "This luminance source does not support cropping.",
        )))
    }

    /**
     * @return Whether this subclass supports counter-clockwise rotation.
     */
    fn is_rotate_supported(&self) -> bool {
        false
    }

    /**
     * @return a wrapper of this {@code LuminanceSource} which inverts the luminances it returns -- black becomes
     *  white and vice versa, and each value becomes (255-value).
     */
    fn invert(&self) -> Result<&Box<dyn LuminanceSource>, Error> {
        todo!()
    }

    /**
     * Returns a new object with rotated image data by 90 degrees counterclockwise.
     * Only callable if {@link #is_rotate_supported()} is true.
     *
     * @return A rotated version of this object.
     */
    fn rotate_counter_clockwise(&self) -> Result<Box<dyn LuminanceSource>, Error> {
        Err(Error::UnsupportedOperationException(String::from(
            "This luminance source does not support rotation by 90 degrees.",
        )))
    }

    /**
     * Returns a new object with rotated image data by 45 degrees counterclockwise.
     * Only callable if {@link #is_rotate_supported()} is true.
     *
     * @return A rotated version of this object.
     */
    fn rotate_counter_clockwise45(&self) -> Result<Box<dyn LuminanceSource>, Error> {
        Err(Error::UnsupportedOperationException(String::from(
            "This luminance source does not support rotation by 45 degrees.",
        )))
    }
}

impl fmt::Display for dyn LuminanceSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.get_width() as usize;
        let height = self.get_height() as usize;
        let mut row = vec![0; width];
        for y in 0..height {
            row = self.get_row(y as i32, &row).unwrap();
            for x in 0..width {
                let luminance = row[x] & 0xFF;
                let c;
                if luminance < 0x40 {
                    c = '#';
                } else if luminance < 0x80 {
                    c = '+';
                } else if luminance < 0xC0 {
                    c = '.';
                } else {
                    c = ' ';
                }
                write!(f, "{}", c).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}
