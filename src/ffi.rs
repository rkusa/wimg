use std::ffi::CString;
use std::os::raw::c_char;

use crate::error::Error;
use crate::Image;

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
pub unsafe extern "C" fn context_destroy(img: *mut Context) {
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
pub unsafe extern "C" fn error_message_destroy(s: *mut c_char) {
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
pub unsafe extern "C" fn image_destroy(img: *mut Image) {
    Box::from_raw(img);
}

#[no_mangle]
pub unsafe extern "C" fn resize(
    ctx: *mut Context,
    img: *mut Image,
    new_width: u32,
    new_height: u32,
    maintain_aspect: bool,
) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let img: &Image = if let Some(img) = img.as_mut() {
        img
    } else {
        ctx.last_error = Some(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::resize::resize(img, new_width, new_height, maintain_aspect) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
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
pub unsafe extern "C" fn jpeg_decode(ctx: *mut Context, ptr: *mut u8, size: usize) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let data = std::slice::from_raw_parts(ptr, size);
    match crate::jpeg::decode(data) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_encode(ctx: *mut Context, img: *mut Image) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        ctx.last_error = Some(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::jpeg::encode(img, &ctx.jpeg_encode_options) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
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
pub unsafe extern "C" fn png_decode(ctx: *mut Context, ptr: *mut u8, size: usize) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let data = std::slice::from_raw_parts(ptr, size);
    match crate::png::decode(data) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_encode(ctx: *mut Context, img: *mut Image) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        ctx.last_error = Some(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::png::encode(img) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avif_seed() -> u32 {
    crate::avif::seed()
}

#[no_mangle]
pub unsafe extern "C" fn avif_encode(ctx: *mut Context, img: *mut Image) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        ctx.last_error = Some(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::avif::encode(img, &ctx.avif_encode_options) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
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
pub unsafe extern "C" fn webp_encode(ctx: *mut Context, img: *mut Image) -> *mut Image {
    let ctx: &mut Context = if let Some(ctx) = ctx.as_mut() {
        ctx
    } else {
        return std::ptr::null_mut();
    };

    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        ctx.last_error = Some(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::webp::encode(img, &ctx.webp_encode_options) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            ctx.last_error = Some(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn webp_set_encode_quality(ctx: *mut Context, quality: u16) {
    if let Some(ctx) = ctx.as_mut() {
        ctx.webp_encode_options.quality = quality;
    }
}
