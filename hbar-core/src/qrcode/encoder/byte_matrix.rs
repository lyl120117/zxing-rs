#[derive(Debug, Clone)]
pub struct ByteMatrix {
    bytes: Vec<Vec<i32>>,
    width: i32,
    height: i32,
}

use std::fmt;
impl fmt::Display for ByteMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            let bytes_y = &self.bytes[y as usize];
            for x in 0..self.width {
                match bytes_y[x as usize] {
                    0 => {
                        write!(f, " 0").unwrap();
                    }
                    1 => {
                        write!(f, " 1").unwrap();
                    }
                    _ => {
                        write!(f, "  ").unwrap();
                    }
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, " {}x{}", self.width, self.height)
    }
}

impl ByteMatrix {
    pub fn new(width: i32, height: i32) -> ByteMatrix {
        if width < 0 || height < 0 {
            panic!()
        }
        ByteMatrix {
            bytes: vec![vec![-1; width as usize]; height as usize],
            width: width,
            height: height,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 {
            panic!()
        }
        return self.bytes[y as usize][x as usize];
    }

    pub fn get_array(&self) -> &Vec<Vec<i32>> {
        &self.bytes
    }

    pub fn set(&mut self, x: i32, y: i32, value: i32) {
        self.bytes[y as usize][x as usize] = value;
    }

    pub fn set_bit(&mut self, x: i32, y: i32, value: bool) {
        if value {
            self.bytes[y as usize][x as usize] = 1
        } else {
            self.bytes[y as usize][x as usize] = 0
        }
    }

    pub fn clear(&mut self, value: i32) {
        for y in 0..self.bytes.len() {
            for i in 0..self.bytes[y].len() {
                self.bytes[y][i] = value;
            }
        }
    }
}
