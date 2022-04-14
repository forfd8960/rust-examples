use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use std::convert::TryFrom;
use photon_rs::transform::SamplingFilter;
use prost::Message;

// declare abi.rs
mod abi;
pub use abi::*;

impl ImageSepc {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSepc> for String {
    fn from(image_sepc: &ImageSepc) -> Self {
        let data = image_sepc.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

impl TryFrom<&str> for ImageSepc {
    type Error  = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSepc::decode(&data[..])?)
    }
}

impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}