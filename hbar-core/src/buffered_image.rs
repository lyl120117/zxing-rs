use crate::Error;

use image::{imageops, ColorType, DynamicImage, GenericImageView, Pixels};

use std::path::Path;

#[derive(Debug, Clone)]
pub struct BufferedImage {
    img: DynamicImage,
}

impl BufferedImage {
    pub const TYPE_BYTE_GRAY: ColorType = ColorType::L8;

    pub fn open(path: &Path) -> Result<BufferedImage, Error> {
        let img = image::open(path);
        let img = match img {
            Ok(img) => img,
            Err(err) => return Err(Error::ImageErrorException(err.to_string())),
        };
        Ok(BufferedImage { img: img })
    }

    pub fn new(width: u32, height: u32, image_type: ColorType) -> Result<BufferedImage, Error> {
        // Create a new ImgBuf with width: imgx and height: imgy
        let img;
        match image_type {
            ColorType::L8 => img = DynamicImage::new_luma8(width, height),
            _ => {
                return Err(Error::ImageErrorException(format!(
                    "Unknown image type {:?}",
                    image_type
                )))
            }
        }

        Ok(BufferedImage { img: img })
    }

    pub fn get_width(&self) -> u32 {
        self.img.width()
    }

    pub fn get_height(&self) -> u32 {
        self.img.height()
    }

    pub fn get_type(&self) -> ColorType {
        self.img.color()
    }

    pub fn get_pixels(&self) -> Pixels<DynamicImage> {
        self.img.pixels()
    }

    pub fn get_rect_data(&self, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
        let mut img = self.img.clone();
        let subimg = imageops::crop(&mut img, x, y, w, h).to_image().to_vec();
        subimg
    }

    pub fn to_gray(&mut self) {
        self.img = image::DynamicImage::ImageLuma8(self.img.to_luma8())
    }

    pub fn rotate90(&mut self) {
        self.img = self.img.rotate90();
    }

    pub fn rotate_by_angle(&mut self, angle: i32) {
        self.img = self.img.huerotate(angle);
    }
}

// RUST_TEST_NOCAPTURE=1 cargo test image_tests
#[cfg(test)]
mod image_tests {
    use super::*;
    #[test]
    #[should_panic]
    fn open_faild() {
        let img = BufferedImage::open(Path::new("out.png")).unwrap();
    }

    #[test]
    fn open() {
        let img = BufferedImage::open(Path::new("../out.png")).unwrap();
        assert_eq!(300, img.get_width());
        assert_eq!(300, img.get_height());

        println!("############# color_type: {:?}", img.get_type());
        println!(
            "############# color_type: {:?}",
            img.get_rect_data(0, 0, 5, 10)
        );
    }
}
