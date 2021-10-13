use crate::{Error, ResultError};

#[derive(Debug, Clone)]
pub struct BitMatrix {
    pub width: i32,
    pub height: i32,
    pub row_size: i32,
    pub bits: Vec<i32>,
}

impl BitMatrix {
    /**
     * Creates an empty square {@code BitMatrix}.
     *
     * @param dimension height and width
     */
    pub fn new1(dimension: i32) -> ResultError<BitMatrix> {
        BitMatrix::new2(dimension, dimension)
    }

    /**
     * Creates an empty {@code BitMatrix}.
     *
     * @param width bit matrix width
     * @param height bit matrix height
     */
    pub fn new2(width: i32, height: i32) -> ResultError<BitMatrix> {
        if width < 1 || height < 1 {
            return Err(Error::IllegalArgumentException(String::from(
                "Both dimensions must be greater than 0",
            )));
        }
        let row_size = (width + 31) / 32;
        Ok(BitMatrix {
            width: width,
            height: height,
            row_size: row_size,
            bits: vec![0; (row_size * height) as usize],
        })
    }

    /**
     * <p>Gets the requested bit, where true means black.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     * @return value of given bit in matrix
     */
    pub fn get(&self, x: u32, y: u32) -> bool {
        let offset = y * self.row_size as u32 + (x / 32);
        ((self.bits[offset as usize] >> (x & 0x1f)) & 1) != 0
    }

    /**
     * <p>Sets the given bit to true.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn set(&mut self, x: u32, y: u32) {
        let offset = y as i32 * self.row_size + (x as i32 / 32);
        self.bits[offset as usize] |= 1 << (x & 0x1f);
    }

    /**
     * <p>Sets a square region of the bit matrix to true.</p>
     *
     * @param left The horizontal position to begin at (inclusive)
     * @param top The vertical position to begin at (inclusive)
     * @param width The width of the region
     * @param height The height of the region
     */
    pub fn set_region(&mut self, left: i32, top: i32, width: i32, height: i32) {
        if top < 0 || left < 0 {
            panic!("Left and top must be nonnegative");
        }
        if height < 1 || width < 1 {
            panic!("Height and width must be at least 1");
        }
        let right = left + width;
        let bottom = top + height;
        // println!("width: {}, height: {}", self.width, self.height);
        // println!("right: {}, bottom: {}", right, bottom);
        if bottom > self.height || right > self.width {
            panic!("The region must fit inside the matrix")
        }
        for y in top..bottom {
            let offset = y * self.row_size;
            for x in left..right {
                self.bits[(offset + (x / 32)) as usize] |= 1 << (x & 0x1f);
            }
        }
    }

    /**
     * This is useful in detecting a corner of a 'pure' barcode.
     *
     * @return {@code x,y} coordinate of top-left-most 1 bit, or null if it is all white
     */
    pub fn getTopLeftOnBit(&self) -> Option<Vec<i32>> {
        let mut bitsOffset = 0;
        while bitsOffset < self.bits.len() && self.bits[bitsOffset] == 0 {
            bitsOffset += 1
        }
        if bitsOffset == self.bits.len() {
            return None;
        }
        let y = bitsOffset as i32 / self.row_size;
        let mut x = (bitsOffset as i32 % self.row_size) * 32;

        let theBits = self.bits[bitsOffset];
        let mut bit = 0;
        while (theBits << (31 - bit)) == 0 {
            bit += 1;
        }
        x += bit;

        Some(vec![x, y])
    }

    pub fn getBottomRightOnBit(&self) -> Option<Vec<i32>> {
        let mut bitsOffset = self.bits.len() - 1;
        while bitsOffset >= 0 && self.bits[bitsOffset] == 0 {
            bitsOffset -= 1;
        }
        if bitsOffset < 0 {
            return None;
        }

        let y = bitsOffset as i32 / self.row_size;
        let mut x = (bitsOffset as i32 % self.row_size) * 32;

        let theBits = self.bits[bitsOffset];
        let mut bit = 31;
        while (theBits >> bit) == 0 {
            bit -= 1;
        }
        x += bit;

        Some(vec![x, y])
    }
}
