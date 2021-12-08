use crate::Image;
use rgb::FromSlice;

pub fn resize(img: &Image, new_width: u32, new_height: u32) -> Image {
    println!(
        "resize {} {} {} {}",
        img.width, img.height, new_width, new_height
    );

    let mut resizer = resize::new(
        img.width as usize,
        img.height as usize,
        new_width as usize,
        new_height as usize,
        resize::Pixel::RGB8,
        resize::Type::Triangle,
    )
    .unwrap();

    let src = img.as_ref();
    let mut dst = vec![0u8; (new_width * new_height * 3) as usize];
    resizer.resize(src.as_rgb(), dst.as_rgb_mut()).unwrap();

    Image::new(dst, new_width, new_height)
}
