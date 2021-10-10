use hbar_core::BufferedImage;
use hbar_core::Error;
use hbar_core::LuminanceSource;

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
        Ok(BufferedImageLuminanceSource {
            image: _image,
            width: width,
            height: height,
            left: left,
            top: top,
        })
    }
}

impl LuminanceSource for BufferedImageLuminanceSource {
    fn get_row(&self, y: i32, _row: Vec<u8>) -> Result<Vec<u8>, Error> {
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
    ) -> Result<Box<dyn LuminanceSource>, Error> {
        Ok(Box::new(BufferedImageLuminanceSource::new1(
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

    fn rotate_counter_clockwise(&self) -> Result<Box<dyn LuminanceSource>, Error> {
        let source_width = self.image.get_width();
        let mut rotated_image = self.image.clone();
        rotated_image.rotate90();
        Ok(Box::new(BufferedImageLuminanceSource::new1(
            &rotated_image,
            self.top,
            source_width - (self.left + self.get_width()),
            self.get_height(),
            self.get_width(),
        )?))
    }

    fn rotate_counter_clockwise45(&self) -> Result<Box<dyn LuminanceSource>, Error> {
        let source_width = self.image.get_width();
        let mut rotated_image = self.image.clone();
        rotated_image.rotate_by_angle(45);
        Ok(Box::new(BufferedImageLuminanceSource::new1(
            &rotated_image,
            self.top,
            source_width - (self.left + self.get_width()),
            self.get_height(),
            self.get_width(),
        )?))
    }
}
