use std::borrow::Cow;

use crate::crop::{crop, Fit};
use crate::error::Error;
use crate::Image;
use rgb::FromSlice;

pub fn resize(img: &Image, new_width: u32, new_height: u32) -> Result<Image, Error> {
    // println!(
    //     "resize {} {} {} {}",
    //     img.width, img.height, new_width, new_height
    // );

    let mut img = Cow::Borrowed(img);
    let aspect_before = f64::from(img.width) / f64::from(img.height);
    let aspect_after = f64::from(new_width) / f64::from(new_height);
    if (aspect_after - aspect_before).abs() >= f64::EPSILON {
        println!(
            "aspect change {} {} -> cropping",
            aspect_before, aspect_after
        );
        let (crop_width, crop_height) = if aspect_after > aspect_before {
            (img.width, (f64::from(img.width) / aspect_after) as u32)
        } else {
            ((f64::from(img.height) * aspect_after) as u32, img.height)
        };
        img = Cow::Owned(crop(&img, crop_width, crop_height, Fit::Contain)?);
    }

    // println!(
    //     "Resize from {}/{} to {}/{}",
    //     img.width, img.height, new_width, new_height
    // );

    let mut resizer = resize::new(
        img.width as usize,
        img.height as usize,
        new_width as usize,
        new_height as usize,
        resize::Pixel::RGB8,
        resize::Type::Triangle,
    )?;

    let src: &[u8] = (&*img).as_ref();
    let mut dst = vec![0u8; (new_width * new_height * 3) as usize];
    resizer.resize(src.as_rgb(), dst.as_rgb_mut())?;

    Ok(Image::new(dst, new_width, new_height))
}
