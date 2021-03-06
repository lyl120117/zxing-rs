use hbar_core::BufferedImage;
use hbar_core::InvertedLuminanceSource;
use hbar_core::LuminanceSource;
use hbar_core::{Error, ResultError};

use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct BufferedImageLuminanceSource {
    image: BufferedImage,
    width: u32,
    height: u32,
    left: u32,
    top: u32,
}

impl BufferedImageLuminanceSource {
    pub fn new(image: &BufferedImage) -> Result<BufferedImageLuminanceSource, Error> {
        BufferedImageLuminanceSource::new1(image, 0, 0, image.get_width(), image.get_height())
    }

    pub fn new1(
        image: &BufferedImage,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> Result<BufferedImageLuminanceSource, Error> {
        let mut _image = (*image).clone();
        if image.get_type() != BufferedImage::TYPE_BYTE_GRAY {
            _image.to_gray()
        }
        println!(
            "width: {}, height: {}, left: {}, top: {}",
            width, height, left, top
        );
        Ok(BufferedImageLuminanceSource {
            image: _image,
            width: width,
            height: height,
            left: left,
            top: top,
        })
    }

    fn clone(&self) -> ResultError<Rc<dyn LuminanceSource>> {
        Ok(Rc::new(BufferedImageLuminanceSource::new1(
            &self.image,
            self.left,
            self.top,
            self.width,
            self.height,
        )?))
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    fn get_row(&self, y: i32, _row: &Vec<u8>) -> Result<Vec<u8>, Error> {
        if y < 0 || y as u32 >= self.get_height() {
            return Err(Error::IllegalArgumentException(format!(
                "Requested row is outside the image: {}",
                y
            )));
        }
        let y = y as u32;
        Ok(self
            .image
            .get_rect_data(self.left, self.top + y, self.get_width(), 1))
    }

    fn get_matrix(&self) -> Result<Vec<u8>, Error> {
        Ok(self
            .image
            .get_rect_data(self.left, self.top, self.get_width(), self.get_height()))
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn is_crop_supported(&self) -> bool {
        true
    }

    fn crop(
        &self,
        _left: u32,
        _top: u32,
        _width: u32,
        _height: u32,
    ) -> ResultError<Rc<dyn LuminanceSource>> {
        Ok(Rc::new(BufferedImageLuminanceSource::new1(
            &self.image,
            self.left + _left,
            self.top + _top,
            _width,
            _height,
        )?))
    }

    /**
     * This is always true, since the image is a gray-scale image.
     *
     * @return true
     */
    fn is_rotate_supported(&self) -> bool {
        true
    }

    fn rotate_counter_clockwise(&self) -> Result<Rc<dyn LuminanceSource>, Error> {
        let source_width = self.image.get_width();
        let mut rotated_image = self.image.clone();
        rotated_image.rotate90();
        Ok(Rc::new(BufferedImageLuminanceSource::new1(
            &rotated_image,
            self.top,
            source_width - (self.left + self.get_width()),
            self.get_height(),
            self.get_width(),
        )?))
    }

    fn rotate_counter_clockwise45(&self) -> Result<Rc<dyn LuminanceSource>, Error> {
        let source_width = self.image.get_width();
        let mut rotated_image = self.image.clone();
        rotated_image.rotate_by_angle(45);
        Ok(Rc::new(BufferedImageLuminanceSource::new1(
            &rotated_image,
            self.top,
            source_width - (self.left + self.get_width()),
            self.get_height(),
            self.get_width(),
        )?))
    }

    fn invert(&self) -> ResultError<Rc<dyn LuminanceSource>> {
        Ok(Rc::new(InvertedLuminanceSource::new(self.clone()?)))
    }
}

impl fmt::Display for BufferedImageLuminanceSource {
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
