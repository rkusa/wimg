#![allow(clippy::missing_safety_doc)]

use mozjpeg_sys::*;
use rgb::FromSlice;
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
        let boxed = Box::new(VecParts {
            ptr: v.as_mut_ptr() as u32,
            len: v.len() as u32,
            cap: v.capacity() as u32,
        });
        Box::into_raw(boxed)
    }
}

#[no_mangle]
unsafe fn decode(offset: u32, size: u32) -> *mut VecParts {
    println!("decode {} {}", offset, size);

    // TODO: error handling?
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);

    jpeg_mem_src(&mut cinfo, offset as *const _, size as c_ulong);
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
unsafe fn encode(offset: u32, size: u32, width: u32, height: u32) -> *mut VecParts {
    println!("encode {} {}", offset, size);

    let mut err = mem::zeroed();
    let mut cinfo: jpeg_compress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_compress(&mut cinfo);

    let mut outsize = 0;
    let mut outbuffer = std::ptr::null_mut();
    jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);

    cinfo.image_width = width;
    cinfo.image_height = height;
    cinfo.in_color_space = J_COLOR_SPACE::JCS_RGB;
    cinfo.input_components = 3;
    jpeg_set_defaults(&mut cinfo);

    let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
    cinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;
    jpeg_set_quality(&mut cinfo, 80, true as boolean);

    jpeg_start_compress(&mut cinfo, true as boolean);

    let buffer = std::slice::from_raw_parts(offset as *const _, size as usize);
    while cinfo.next_scanline < cinfo.image_height {
        let offset = cinfo.next_scanline as usize * row_stride;
        let jsamparray = [buffer[offset..].as_ptr()];
        jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1);
    }

    jpeg_finish_compress(&mut cinfo);
    jpeg_destroy_compress(&mut cinfo);

    let buffer = std::slice::from_raw_parts(outbuffer, outsize as usize).to_vec();
    VecParts::new(buffer)
}

#[no_mangle]
fn resize(offset: u32, size: u32, w1: u32, h1: u32, w2: u32, h2: u32) -> *mut VecParts {
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
