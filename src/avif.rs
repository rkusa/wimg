use rgb::FromSlice;

use crate::error::Error;
use crate::{Image, ImageFormat};

pub fn seed() -> u32 {
    1
}

#[derive(Debug, Clone)]
pub struct EncodeOptions {
    /// 0-100 scale
    pub quality: u16,
    /// rav1e preset 1 (slow) 10 (fast but crappy)
    pub speed: u8,
}

pub fn encode(img: &Image, opts: &EncodeOptions) -> Result<Image, Error> {
    let enc = ravif::Encoder::new()
        .with_quality(opts.quality as f32)
        .with_alpha_quality(opts.quality as f32)
        .with_speed(opts.speed)
        .with_internal_color_model(ravif::ColorModel::YCbCr)
        .with_num_threads(cfg!(target_family = "wasm").then_some(1));
    match img.format {
        ImageFormat::RGB8 => {
            let input = ravif::Img::new(
                img.as_ref().as_rgb(),
                img.width as usize,
                img.height as usize,
            );
            let ravif::EncodedImage { avif_file, .. } =
                enc.encode_rgb(input).map_err(Error::Avif)?;
            Ok(Image::new(
                avif_file,
                ImageFormat::AVIF,
                img.width,
                img.height,
            ))
        }
        ImageFormat::RGBA8 => {
            let data = img.as_ref().as_rgba().to_vec();
            let input = ravif::Img::new(data, img.width as usize, img.height as usize);
            let enc = enc.with_alpha_color_mode(ravif::AlphaColorMode::UnassociatedClean);
            let ravif::EncodedImage { avif_file, .. } =
                enc.encode_rgba(input.as_ref()).map_err(Error::Avif)?;
            Ok(Image::new(
                avif_file,
                ImageFormat::AVIF,
                img.width,
                img.height,
            ))
        }
        _ => Err(Error::Process {
            process: "encode as AVIF",
            format: img.format,
        }),
    }
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            quality: 60,
            speed: 5,
        }
    }
}
