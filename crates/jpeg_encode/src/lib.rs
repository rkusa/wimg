#![allow(clippy::missing_safety_doc)]

use std::mem;

use mozjpeg_sys::*;
use wimg_common::VecParts;

#[no_mangle]
unsafe fn encode(offset: u32, size: u32, width: u32, height: u32) -> *mut VecParts {
    println!("encode {} {} {} {}", offset, size, width, height);

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

    // println!("A");
    jpeg_finish_compress(&mut cinfo);
    // println!("B");
    jpeg_destroy_compress(&mut cinfo);
    // println!("C {} {}", outbuffer as u32, outsize);

    let buffer = std::slice::from_raw_parts(outbuffer, outsize as usize).to_vec();
    // println!("D");
    VecParts::new(buffer)
}
