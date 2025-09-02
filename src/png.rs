use std::io::Cursor;

use crate::error::Error;
use crate::{Image, ImageFormat};
use png::{BitDepth, ColorType, Decoder, Encoder, Transformations};

pub fn seed() -> u32 {
    1
}

pub fn decode(data: &[u8]) -> Result<Image, Error> {
    let data = Cursor::new(data);
    let mut decoder = Decoder::new(data);
    decoder.set_transformations(Transformations::STRIP_16 | Transformations::EXPAND);
    let mut reader = decoder.read_info().map_err(PngError::from)?;
    if reader.info().frame_control.is_some() {
        return Err(PngError::UnsupportedAnimation.into());
    }

    let mut buf = vec![0; reader.output_buffer_size().ok_or(Error::ExceedsMemory)?];
    let info = reader.next_frame(&mut buf).map_err(PngError::from)?;

    let image_format = match info.color_type {
        ColorType::Rgb => ImageFormat::RGB8,
        ColorType::Rgba => ImageFormat::RGBA8,
        _ => return Err(PngError::UnsupportedColorType(info.color_type).into()),
    };

    buf.resize(info.buffer_size(), 0);

    Ok(Image::new(buf, image_format, info.width, info.height))
}

pub fn encode(img: &Image) -> Result<Image, Error> {
    let mut buf = Vec::new();
    let mut encoder = Encoder::new(&mut buf, img.width, img.height);
    encoder.set_color(match img.format {
        ImageFormat::RGB8 => ColorType::Rgb,
        ImageFormat::RGBA8 => ColorType::Rgba,
        _ => return Err(PngError::InvalidSource(img.format).into()),
    });
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(PngError::from)?;
    writer
        .write_image_data(img.as_ref())
        .map_err(PngError::from)?;
    std::mem::drop(writer);

    Ok(Image::new(buf, ImageFormat::PNG, img.width, img.height))
}

#[derive(Debug, thiserror::Error)]
pub enum PngError {
    #[error("failed to decode PNG image")]
    Decode(#[from] png::DecodingError),
    #[error("failed to encode image as PNG")]
    Encode(#[from] png::EncodingError),
    #[error("unsupported color type: {0:?}")]
    UnsupportedColorType(ColorType),
    #[error("animated PNGs are not supported")]
    UnsupportedAnimation,
    #[error("cannot encode {0} as PNG")]
    InvalidSource(ImageFormat),
}
