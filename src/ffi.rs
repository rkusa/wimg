use crate::Image;

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
pub unsafe extern "C" fn resize(img: *mut Image, new_width: u32, new_height: u32) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        return std::ptr::null_mut();
    };
    crate::resize::resize(img, new_width, new_height).into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_decode(ptr: *mut u8, size: usize) -> *mut Image {
    crate::jpeg::decode(ptr, size).into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn jpeg_encode(img: *mut Image) -> *mut Image {
    let img: &Image = if let Some(img) = img.as_ref() {
        img
    } else {
        return std::ptr::null_mut();
    };
    crate::jpeg::encode(img).into_raw()
}
