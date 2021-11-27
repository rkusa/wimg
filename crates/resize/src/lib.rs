#![allow(clippy::missing_safety_doc)]

use rgb::FromSlice;
use wimg_common::VecParts;

#[no_mangle]
fn resize(offset: u32, size: u32, w1: u32, h1: u32, w2: u32, h2: u32) -> *mut VecParts {
    println!("resize {} {} {} {} {} {}", offset, size, w1, h1, w2, h2);

    let mut resizer = resize::new(
        w1 as usize,
        h1 as usize,
        w2 as usize,
        h2 as usize,
        resize::Pixel::RGB8,
        resize::Type::Triangle,
    )
    .unwrap();

    let src = unsafe { std::slice::from_raw_parts(offset as *const _, size as usize) };
    let mut dst = vec![0u8; (w2 * h2 * 3) as usize];
    resizer.resize(src.as_rgb(), dst.as_rgb_mut()).unwrap();

    VecParts::new(dst)
}
