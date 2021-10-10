#[derive(Debug, Clone)]
pub struct BitMatrix {
    pub width: i32,
    pub height: i32,
    pub row_size: i32,
    pub bits: Vec<i32>,
}

impl BitMatrix {
    pub fn new2(width: i32, height: i32) -> BitMatrix {
        if width < 1 || height < 1 {
            panic!("Both dimensions must be greater than 0");
        }
        let row_size = (width + 31) / 32;
        BitMatrix {
            width: width,
            height: height,
            row_size: row_size,
            bits: vec![0; (row_size * height) as usize],
        }
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
}
