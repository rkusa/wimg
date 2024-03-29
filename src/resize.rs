use std::borrow::Cow;

use crate::crop::{crop, Fit};
use crate::error::Error;
use crate::{Image, ImageFormat, PixelFormat};
use rgb::FromSlice;

pub fn seed() -> u32 {
    1
}

pub fn resize(
    img: &Image,
    mut new_width: u32,
    mut new_height: u32,
    maintain_aspect: bool,
) -> Result<Image, Error> {
    // println!(
    //     "resize {} {} {} {}",
    //     img.width, img.height, new_width, new_height
    // );

    let pixel_format = match img.format {
        ImageFormat::RGB8 => PixelFormat::RGB8,
        ImageFormat::RGBA8 => PixelFormat::RGBA8,
        _ => {
            return Err(Error::Process {
                process: "resize",
                format: img.format,
            })
        }
    };

    if new_width > img.width {
        new_width = img.width;
    }
    if new_height > img.height {
        new_height = img.height;
    }

    let mut img = Cow::Borrowed(img);

    // TODO: checked div?

    let aspect_before = f64::from(img.width) / f64::from(img.height);
    let aspect_after = f64::from(new_width) / f64::from(new_height);

    if (aspect_after - aspect_before).abs() >= f64::EPSILON {
        if maintain_aspect {
            if aspect_after > aspect_before {
                let scale = f64::from(new_height) / f64::from(img.height);
                new_width = (f64::from(img.width) * scale) as u32;
            } else {
                let scale = f64::from(new_width) / f64::from(img.width);
                new_height = (f64::from(img.height) * scale) as u32;
            }
        } else {
            // println!(
            //     "aspect change {} {} -> cropping",
            //     aspect_before, aspect_after
            // );
            let (crop_width, crop_height) = if aspect_after > aspect_before {
                (img.width, (f64::from(img.width) / aspect_after) as u32)
            } else {
                ((f64::from(img.height) * aspect_after) as u32, img.height)
            };
            img = Cow::Owned(crop(&img, crop_width, crop_height, Fit::Contain)?);
        }
    }

    // println!(
    //     "Resize from {}/{} to {}/{}",
    //     img.width, img.height, new_width, new_height
    // );

    let src: &[u8] = (*img).as_ref();
    let dst = match pixel_format {
        PixelFormat::RGB8 => {
            let mut resizer = resize::new(
                img.width as usize,
                img.height as usize,
                new_width as usize,
                new_height as usize,
                resize::Pixel::RGB8,
                resize::Type::Triangle,
            )?;

            let mut dst = vec![0u8; (new_width * new_height) as usize * pixel_format.pixel_size()];
            resizer.resize(src.as_rgb(), dst.as_rgb_mut())?;
            dst
        }
        PixelFormat::RGBA8 => {
            let mut resizer = resize::new(
                img.width as usize,
                img.height as usize,
                new_width as usize,
                new_height as usize,
                resize::Pixel::RGBA8,
                resize::Type::Triangle,
            )?;

            let mut dst = vec![0u8; (new_width * new_height) as usize * pixel_format.pixel_size()];
            resizer.resize(src.as_rgba(), dst.as_rgba_mut())?;
            dst
        }
    };

    Ok(Image::new(dst, img.format, new_width, new_height))
}
