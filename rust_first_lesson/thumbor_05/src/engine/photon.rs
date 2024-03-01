use std::io::Cursor;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use photon_rs::{effects, filters, multiple, native::open_image_from_bytes, transform, PhotonImage};
use lazy_static::lazy_static;
use crate::pb::*;

use super::{Engine, SpecTransform};

lazy_static! {
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[crate::Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Filph(ref v)) => self.transform(v),
                Some(spec::Data::Filpv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self, format: image::ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        self.0 = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
    }
}

impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0);
    }
}

impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0);
    }
}

impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::try_from(op.filter).unwrap() {
            filter::Filter::Unspecified => {}
            f => filters::filter(&mut self.0, f.to_str().unwrap()),
        }
    }
}

impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::ResizeType::try_from(op.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(&mut self.0, op.width, op.height, resize::SampleFilter::try_from(op.filter).unwrap().into()),
            resize::ResizeType::SeamCarve => transform::seam_carve(&mut self.0, op.width, op.height)
        };
        self.0 = img;
    }
}

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}

fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = DynamicImage::ImageRgb8(img_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dynimage.write_to(&mut Cursor::new(&mut buffer), format).unwrap();
    buffer
}