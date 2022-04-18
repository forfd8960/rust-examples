use lazy_static::LazyStatic;
use photon_rs::{
    PhotonImage,
    native::open_image_from_bytes,
    transform,
};

lazy_static! {
    static ref WATERMARK: PhotonImage  = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);