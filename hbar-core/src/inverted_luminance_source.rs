use std::borrow::Borrow;

use crate::Error;
use crate::LuminanceSource;

use std::rc::Rc;

pub struct InvertedLuminanceSource {
    delegate: Rc<dyn LuminanceSource>,
}

impl InvertedLuminanceSource {
    pub fn new(delegate: Rc<dyn LuminanceSource>) -> InvertedLuminanceSource {
        InvertedLuminanceSource { delegate: delegate }
    }
}

impl LuminanceSource for InvertedLuminanceSource {
    fn get_row(&self, y: i32, row: &Vec<u8>) -> Result<Vec<u8>, Error> {
        let mut row = self.delegate.get_row(y, row)?;
        let width = self.get_width();
        for i in 0..width {
            let i = i as usize;
            row[i] = 255 - (row[i] & 0xFF);
        }
        Ok(row)
    }

    fn get_matrix(&self) -> Result<Vec<u8>, Error> {
        let matrix = self.delegate.get_matrix()?;
        let length = (self.get_width() * self.get_height()) as usize;
        let mut inverted_matrix = vec![0; length];
        for i in 0..length {
            inverted_matrix[i] = 255 - (matrix[i] & 0xFF)
        }
        Ok(inverted_matrix)
    }

    fn get_width(&self) -> u32 {
        self.delegate.get_width()
    }

    fn get_height(&self) -> u32 {
        self.delegate.get_height()
    }

    fn is_crop_supported(&self) -> bool {
        self.delegate.is_crop_supported()
    }

    fn crop(
        &self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> Result<Rc<dyn LuminanceSource>, Error> {
        Ok(Rc::new(InvertedLuminanceSource::new(
            self.delegate.crop(left, top, width, height)?,
        )))
    }

    fn is_rotate_supported(&self) -> bool {
        self.delegate.is_rotate_supported()
    }

    fn invert(&self) -> Result<Rc<dyn LuminanceSource>, Error> {
        Ok(Rc::clone(&self.delegate))
    }

    fn rotate_counter_clockwise(&self) -> Result<Rc<dyn LuminanceSource>, Error> {
        Ok(Rc::new(InvertedLuminanceSource::new(
            self.delegate.rotate_counter_clockwise()?,
        )))
    }

    fn rotate_counter_clockwise45(&self) -> Result<Rc<dyn LuminanceSource>, Error> {
        Ok(Rc::new(InvertedLuminanceSource::new(
            self.delegate.rotate_counter_clockwise45()?,
        )))
    }
}
