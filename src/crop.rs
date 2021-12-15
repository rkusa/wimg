use crate::error::Error;
use crate::{Image, ImageFormat, PixelFormat};

#[repr(u8)]
pub enum Fit {
    Contain = 0,
    // Stretch = 1,
}

pub fn crop(img: &Image, width: u32, height: u32, fit: Fit) -> Result<Image, Error> {
    if width == 0 || height == 0 || width > img.width || height > img.height {
        return Err(Error::CropOutOfBounds);
    }

    let pixel_format = match img.format {
        ImageFormat::RGB8 => PixelFormat::RGB8,
        ImageFormat::RGBA8 => PixelFormat::RGBA8,
        _ => {
            return Err(Error::Process {
                process: "crop",
                format: img.format,
            })
        }
    };

    let (crop_x, crop_y) = match fit {
        Fit::Contain => ((img.width - width) / 2, (img.height - height) / 2),
        // Fit::Stretch => (0, 0),
    };

    let src: &[u8] = img.as_ref();
    let mut dst = Vec::with_capacity((width * height) as usize * pixel_format.pixel_size());

    for y in 0..height {
        let from_start = ((y + crop_y) * img.width + crop_x) as usize * pixel_format.pixel_size();
        let from_end = from_start + (width as usize) * pixel_format.pixel_size();
        dst.extend_from_slice(&src[from_start..from_end]);
    }

    Ok(Image::new(dst, img.format, width, height))
}
