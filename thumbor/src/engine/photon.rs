use super::{Engine, SpecTransform};
use crate::pb::*;
use lazy_static::lazy_static;
use photon_rs::{
    PhotonImage,
    native::open_image_from_bytes,
    transform,
};
use image::{ImageOutputFormat, DynamicImage, ImageBuffer};


lazy_static! {
    static ref WATERMARK: PhotonImage  = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&data)))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {

            }
        }
    }

    fn generate(self, format: ImageOutputFormat) Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpecTransform<&Crop> for Photon {

}

impl SpecTransform<&Contrast> for Photon {

}

impl SpecTransform<&Flipv> for Photon {

}

impl SpecTransform<&Fliph> for Photon {

}

impl SpecTransform<&Filter> for Photon {

}

impl SpecTransform<&Resize> for Photon {

}

impl SpecTransform<&Watermark> for Photon {

}

fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynamic_image = DynamicImage::ImageRgba8(img_buffer);
    let mut buffer = Vec::with_capacity(32768);
    dynamic_image.write_to(&mut buffer, format).unwrap();
    buffer
}