use crate::{Image, ImageFormat};
use png::{BitDepth, ColorType, Decoder, Encoder, Transformations};

pub fn decode(data: &[u8]) -> Result<Image, PngError> {
    let mut decoder = Decoder::new(data);
    decoder.set_transformations(Transformations::STRIP_16 | Transformations::EXPAND);
    let mut reader = decoder.read_info()?;
    if reader.info().frame_control.is_some() {
        return Err(PngError::UnsupportedAnimation);
    }

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    let image_format = match info.color_type {
        ColorType::Rgb => ImageFormat::RGB8,
        ColorType::Rgba => ImageFormat::RGBA8,
        _ => return Err(PngError::UnsupportedColorType(info.color_type)),
    };

    buf.resize(info.buffer_size(), 0);

    Ok(Image::new(buf, image_format, info.width, info.height))
}

pub fn encode(img: &Image) -> Result<Image, PngError> {
    let mut buf = Vec::new();
    let mut encoder = Encoder::new(&mut buf, img.width, img.height);
    encoder.set_color(match img.format {
        ImageFormat::RGB8 => ColorType::Rgb,
        ImageFormat::RGBA8 => ColorType::Rgba,
        _ => return Err(PngError::InvalidSource(img.format)),
    });
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(img.as_ref())?;
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
