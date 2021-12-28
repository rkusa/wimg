use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::error::Error;
use crate::Image;

// TODO: move into context?
thread_local! {
    static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
    static JPEG_ENCODE_OPTIONS: RefCell<crate::jpeg::EncodeOptions> = RefCell::new(Default::default());
    static AVIF_ENCODE_OPTIONS: RefCell<crate::avif::EncodeOptions> = RefCell::new(Default::default());
    static WEBP_ENCODE_OPTIONS: RefCell<crate::webp::EncodeOptions> = RefCell::new(Default::default());
}

fn update_last_error(err: Error) {
    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(err);
    });
}

fn take_last_error() -> Option<Error> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

#[no_mangle]
pub unsafe extern "C" fn last_error_message() -> *mut c_char {
    use std::fmt::Write;

    if let Some(err) = take_last_error() {
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
    img: *mut Image,
    new_width: u32,
    new_height: u32,
    maintain_aspect: bool,
) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_mut() {
        img
    } else {
        update_last_error(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::resize::resize(img, new_width, new_height, maintain_aspect) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err);
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

    if let Err(err) = out.write_all(&hash) {
        update_last_error(Error::Io(err));
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_seed() -> u32 {
    crate::jpeg::seed()
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_decode(ptr: *mut u8, size: usize) -> *mut Image {
    let data = std::slice::from_raw_parts(ptr, size);
    match crate::jpeg::decode(data) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_encode(img: *mut Image) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        update_last_error(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::jpeg::encode(img, &JPEG_ENCODE_OPTIONS.with(|o| o.borrow().clone())) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_set_encode_quality(quality: u16) {
    JPEG_ENCODE_OPTIONS.with(|opts| {
        let mut opts = opts.borrow_mut();
        opts.quality = quality;
    });
}

#[no_mangle]
pub unsafe extern "C" fn png_seed() -> u32 {
    crate::png::seed()
}

#[no_mangle]
pub unsafe extern "C" fn png_decode(ptr: *mut u8, size: usize) -> *mut Image {
    let data = std::slice::from_raw_parts(ptr, size);
    match crate::png::decode(data) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err.into());
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_encode(img: *mut Image) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        update_last_error(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::png::encode(img) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err.into());
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avif_seed() -> u32 {
    crate::avif::seed()
}

#[no_mangle]
pub unsafe extern "C" fn avif_encode(img: *mut Image) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        update_last_error(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::avif::encode(img, &AVIF_ENCODE_OPTIONS.with(|o| o.borrow().clone())) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn avif_set_encode_quality(quality: u16) {
    AVIF_ENCODE_OPTIONS.with(|opts| {
        let mut opts = opts.borrow_mut();
        opts.quality = quality;
    });
}

#[no_mangle]
pub unsafe extern "C" fn avif_set_encode_speed(speed: u8) {
    AVIF_ENCODE_OPTIONS.with(|opts| {
        let mut opts = opts.borrow_mut();
        opts.speed = speed;
    });
}

#[no_mangle]
pub unsafe extern "C" fn webp_seed() -> u32 {
    crate::webp::seed()
}

#[no_mangle]
pub unsafe extern "C" fn webp_encode(img: *mut Image) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        update_last_error(Error::NullPtr);
        return std::ptr::null_mut();
    };

    match crate::webp::encode(img, &WEBP_ENCODE_OPTIONS.with(|o| o.borrow().clone())) {
        Ok(img) => img.into_raw(),
        Err(err) => {
            update_last_error(err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn webp_set_encode_quality(quality: u16) {
    WEBP_ENCODE_OPTIONS.with(|opts| {
        let mut opts = opts.borrow_mut();
        opts.quality = quality;
    });
}
