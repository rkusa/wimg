use rgb::FromSlice;

use crate::error::Error;
use crate::{Image, ImageFormat};

pub fn encode(img: &Image) -> Result<Image, Error> {
    let config = ravif::Config {
        quality: 80.0,
        alpha_quality: 80.0,
        speed: 1,
        premultiplied_alpha: false,
        color_space: ravif::ColorSpace::YCbCr,
        #[cfg(target_family = "wasm")]
        threads: 1,
        #[cfg(not(target_family = "wasm"))]
        threads: 0,
    };
    match img.format {
        ImageFormat::RGB8 => {
            let input = ravif::Img::new(
                img.as_ref().as_rgb(),
                img.width as usize,
                img.height as usize,
            );
            let (data, _) = ravif::encode_rgb(input, &config).map_err(Error::Avif)?;
            Ok(Image::new(data, ImageFormat::AVIF, img.width, img.height))
        }
        ImageFormat::RGBA8 => {
            let data = img.as_ref().as_rgba().to_vec();
            let input = ravif::Img::new(data, img.width as usize, img.height as usize);
            let input = ravif::cleared_alpha(input);
            let (data, _, _) = ravif::encode_rgba(input.as_ref(), &config).map_err(Error::Avif)?;
            Ok(Image::new(data, ImageFormat::AVIF, img.width, img.height))
        }
        _ => Err(Error::Process {
            process: "encode as AVIF",
            format: img.format,
        }),
    }
}
