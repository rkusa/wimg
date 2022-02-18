use std::ffi::CString;
use std::os::raw::c_char;

use crate::error::Error;
use crate::Image;

macro_rules! as_mut {
    ($expr:expr $(,)?) => {
        if let Some(out) = $expr.as_mut() {
            out
        } else {
            return ErrorCode::NullPtr as i32;
        }
    };
}

#[derive(Default)]
pub struct Context {
    last_error: Option<Error>,
    jpeg_encode_options: crate::jpeg::EncodeOptions,
    avif_encode_options: crate::avif::EncodeOptions,
    webp_encode_options: crate::webp::EncodeOptions,
}

#[no_mangle]
pub unsafe extern "C" fn context_new() -> *mut Context {
    Box::into_raw(Box::new(Context::default()))
}

#[no_mangle]
pub unsafe extern "C" fn context_drop(img: *mut Context) {
    Box::from_raw(img);
}

#[no_mangle]
pub unsafe extern "C" fn last_error_message(ctx: *mut Context) -> *mut c_char {
    use std::fmt::Write;

    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    if let Some(err) = ctx.last_error.take() {
        let mut message = err.to_string();

        let mut source = std::error::Error::source(&err);
        let mut i = 0;

        if source.is_some() {
            message += "\n\nCaused by:\n";
        }

        while let Some(err) = source {
            if i > 0 {
                writeln!(&mut message).ok();
            }
            write!(&mut message, "{:>4}: {}", i, err).ok();
            source = std::error::Error::source(err);
            i += 1;
        }

        CString::new(message).unwrap().into_raw()
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn error_message_drop(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}

#[cfg(target_family = "wasm")]
#[no_mangle]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc, Layout};
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    alloc(layout)
}

#[cfg(target_family = "wasm")]
#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    use std::alloc::{dealloc, Layout};
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn image_new() -> *mut Image {
    Image::new(Vec::new(), crate::ImageFormat::RGB8, 0, 0).into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn image_drop(img: *mut Image) {
    Box::from_raw(img);
}

#[no_mangle]
pub unsafe extern "C" fn resize(
    ctx: *mut Context,
    img: *mut Image,
    new_width: u32,
    new_height: u32,
    maintain_aspect: bool,
    out: *mut Image,
) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let img: &mut Image = as_mut!(img);
    let out: &mut Image = as_mut!(out);

    match crate::resize::resize(img, new_width, new_height, maintain_aspect) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Resize as i32
        }
    }
}

#[cfg(not(target_family = "wasm"))]
#[no_mangle]
pub unsafe extern "C" fn hash(ptr: *mut u8, size: usize, seed: u32) -> u64 {
    let data = std::slice::from_raw_parts(ptr, size);
    crate::hash::hash(data, seed)
}

#[cfg(target_family = "wasm")]
#[no_mangle]
pub unsafe fn hash(ptr: *mut u8, size: usize, seed: u32, out: *mut u8) {
    use std::io::Write;

    let data = std::slice::from_raw_parts(ptr, size);

    let hash = crate::hash::hash(data, seed).to_be_bytes();
    let mut out = std::slice::from_raw_parts_mut(out, hash.len());

    out.write_all(&hash).ok();
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_seed() -> u32 {
    crate::jpeg::seed()
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_decode(
    ctx: *mut Context,
    ptr: *const u8,
    size: usize,
    out: *mut Image,
) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let out: &mut Image = as_mut!(out);
    if ptr.is_null() {
        return ErrorCode::NullPtr as i32;
    }

    let data = std::slice::from_raw_parts(ptr, size);
    match crate::jpeg::decode(data) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Decode as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_encode(ctx: *mut Context, img: *mut Image, out: *mut Image) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let img: &mut Image = as_mut!(img);
    let out: &mut Image = as_mut!(out);

    match crate::jpeg::encode(img, &ctx.jpeg_encode_options) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Encode as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_set_encode_quality(ctx: *mut Context, quality: u16) {
    if let Some(ctx) = ctx.as_mut() {
        ctx.jpeg_encode_options.quality = quality;
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_seed() -> u32 {
    crate::png::seed()
}

#[no_mangle]
pub unsafe extern "C" fn png_decode(
    ctx: *mut Context,
    ptr: *const u8,
    size: usize,
    out: *mut Image,
) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let out: &mut Image = as_mut!(out);
    if ptr.is_null() {
        return ErrorCode::NullPtr as i32;
    }

    let data = std::slice::from_raw_parts(ptr, size);
    match crate::png::decode(data) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Decode as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_encode(ctx: *mut Context, img: *mut Image, out: *mut Image) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let img: &mut Image = as_mut!(img);
    let out: &mut Image = as_mut!(out);

    match crate::png::encode(img) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Encode as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avif_seed() -> u32 {
    crate::avif::seed()
}

#[no_mangle]
pub unsafe extern "C" fn avif_encode(ctx: *mut Context, img: *mut Image, out: *mut Image) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let img: &mut Image = as_mut!(img);
    let out: &mut Image = as_mut!(out);

    match crate::avif::encode(img, &ctx.avif_encode_options) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Decode as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avif_set_encode_quality(ctx: *mut Context, quality: u16) {
    if let Some(ctx) = ctx.as_mut() {
        ctx.avif_encode_options.quality = quality;
    }
}

#[no_mangle]
pub unsafe extern "C" fn avif_set_encode_speed(ctx: *mut Context, speed: u8) {
    if let Some(ctx) = ctx.as_mut() {
        ctx.avif_encode_options.speed = speed;
    }
}

#[no_mangle]
pub unsafe extern "C" fn webp_seed() -> u32 {
    crate::webp::seed()
}

#[no_mangle]
pub unsafe extern "C" fn webp_encode(ctx: *mut Context, img: *mut Image, out: *mut Image) -> i32 {
    let ctx: &mut Context = as_mut!(ctx);
    let img: &mut Image = as_mut!(img);
    let out: &mut Image = as_mut!(out);

    match crate::webp::encode(img, &ctx.webp_encode_options) {
        Ok(img) => {
            *out = img;
            0
        }
        Err(err) => {
            ctx.last_error = Some(err);
            ErrorCode::Decode as i32
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn webp_set_encode_quality(ctx: *mut Context, quality: u16) {
    if let Some(ctx) = ctx.as_mut() {
        ctx.webp_encode_options.quality = quality;
    }
}

#[repr(i32)]
pub enum ErrorCode {
    /// Received an unexpected null pointer.
    NullPtr = -1,

    /// Failed to decode image.
    Decode = -2,

    /// Failed to encode image.
    Encode = -3,

    /// Failed to resize image.
    Resize = -4,
}
