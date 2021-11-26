#![allow(clippy::missing_safety_doc)]

use mozjpeg_sys::*;
use std::mem::{self, ManuallyDrop};
use std::os::raw::c_ulong;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[repr(C)]
#[derive(Debug)]
pub struct VecParts {
    ptr: u32,
    len: u32,
    cap: u32,
}

impl VecParts {
    pub fn new(v: Vec<u8>) -> *mut VecParts {
        let mut v = ManuallyDrop::new(v);
        let boxed = Box::new(VecParts { ptr:v.as_mut_ptr() as u32, len: v.len() as u32, cap: v.capacity() as u32 });
        println!("VecParts: {:?}", boxed);
        Box::into_raw(boxed)
    }
}

#[no_mangle]
unsafe fn decode(offset: i32, size: i32) -> *mut VecParts {
    println!("decode {} {}", offset, size);

    // TODO: error handling?
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);

    jpeg_mem_src(&mut cinfo, offset as * const _, size as c_ulong);
    jpeg_read_header(&mut cinfo, true as boolean);

    cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
    jpeg_start_decompress(&mut cinfo);

    let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
    let buffer_size = row_stride * cinfo.image_height as usize;
    let mut buffer = vec![0u8; buffer_size];

    while cinfo.output_scanline < cinfo.output_height {
        let offset = cinfo.output_scanline as usize * row_stride;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);
    }

    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);

    VecParts::new(buffer)
}

#[no_mangle]
pub unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc, Layout};

    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    alloc(layout)
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    use std::alloc::{dealloc, Layout};
    let align = std::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe fn dealloc_vec(ptr: *mut VecParts) {
    let boxed: Box<VecParts> = Box::from_raw(ptr);
    Vec::from_raw_parts(boxed.ptr as *mut u8, boxed.len as usize, boxed.cap as usize);
}
