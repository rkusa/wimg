#![allow(clippy::missing_safety_doc)]

use std::mem::{self, size_of};

use mozjpeg_sys::*;
use wimg_common::VecParts;

#[no_mangle]
pub unsafe fn decode(offset: u32, size: u32) -> *mut VecParts {
    println!("decode {} {}", offset, size);

    // TODO: error handling?
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);

    jpeg_mem_src(&mut cinfo, offset as *const _, size as c_ulong);
    jpeg_read_header(&mut cinfo, true as boolean);

    println!("width={}, height={}", cinfo.image_width, cinfo.image_height);

    cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
    jpeg_start_decompress(&mut cinfo);

    let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
    let buffer_size = row_stride * cinfo.image_height as usize;
    let dim_size = size_of::<u32>() * 2;
    let mut buffer = vec![0u8; buffer_size + dim_size];

    // write dimensions
    buffer[..4].copy_from_slice(&cinfo.image_width.to_be_bytes()[..]);
    buffer[4..8].copy_from_slice(&cinfo.image_height.to_be_bytes()[..]);

    while cinfo.output_scanline < cinfo.output_height {
        let offset = dim_size + cinfo.output_scanline as usize * row_stride;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);
    }

    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);

    VecParts::new(buffer)
}
