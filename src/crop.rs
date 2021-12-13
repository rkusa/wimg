use crate::error::Error;
use crate::Image;

#[repr(u8)]
pub enum Fit {
    Contain = 0,
    // Stretch = 1,
}

pub fn crop(img: &Image, width: u32, height: u32, fit: Fit) -> Result<Image, Error> {
    if width == 0 || height == 0 || width > img.width || height > img.height {
        return Err(Error::CropOutOfBounds);
    }

    let (crop_x, crop_y) = match fit {
        Fit::Contain => ((img.width - width) / 2, (img.height - height) / 2),
        // Fit::Stretch => (0, 0),
    };

    let src: &[u8] = img.as_ref();
    let mut dst = Vec::with_capacity((width * height * 3) as usize); // 3 -> RGB

    for y in 0..height {
        let from_start = ((y + crop_y) * 3 * img.width + crop_x * 3) as usize;
        let from_end = from_start + (width as usize) * 3;
        dst.extend_from_slice(&src[from_start..from_end]);
    }

    Ok(Image::new(dst, width, height))
}
