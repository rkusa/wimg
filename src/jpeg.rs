use crate::error::Error;
use crate::Image;
use mozjpeg_sys::*;

pub fn decode(ptr: *mut u8, size: usize) -> Result<Image, Error> {
    // println!("decode");

    std::panic::catch_unwind(|| unsafe {
        let mut cinfo: jpeg_decompress_struct = std::mem::zeroed();

        let mut err: jpeg_error_mgr = std::mem::zeroed();
        jpeg_std_error(&mut err);
        err.error_exit = Some(error_exit);
        err.output_message = Some(output_message);
        cinfo.common.err = &mut err;

        jpeg_create_decompress(&mut cinfo);

        jpeg_mem_src(&mut cinfo, ptr, size as c_ulong);
        jpeg_read_header(&mut cinfo, true as boolean);

        // println!("width={}, height={}", cinfo.image_width, cinfo.image_height);

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

        Image::new(buffer, cinfo.image_width, cinfo.image_height)
    })
    .map_err(|err| Error::Jpeg(err))
}

pub fn encode(img: &Image) -> Result<Image, Error> {
    // println!("encode {} {}", img.width, img.height);

    std::panic::catch_unwind(|| unsafe {
        let mut cinfo: jpeg_compress_struct = std::mem::zeroed();

        let mut err: jpeg_error_mgr = std::mem::zeroed();
        jpeg_std_error(&mut err);
        err.error_exit = Some(error_exit);
        err.output_message = Some(output_message);
        cinfo.common.err = &mut err;

        jpeg_create_compress(&mut cinfo);

        let mut outsize = 0;
        let mut outbuffer = std::ptr::null_mut();
        jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);

        cinfo.image_width = img.width;
        cinfo.image_height = img.height;
        cinfo.in_color_space = J_COLOR_SPACE::JCS_RGB;
        cinfo.input_components = 3;
        jpeg_set_defaults(&mut cinfo);

        let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
        cinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;
        jpeg_set_quality(&mut cinfo, 80, true as boolean);

        jpeg_start_compress(&mut cinfo, true as boolean);

        let buffer = img.as_ref();
        while cinfo.next_scanline < cinfo.image_height {
            let offset = cinfo.next_scanline as usize * row_stride;
            let jsamparray = [buffer[offset..].as_ptr()];
            jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1);
        }

        jpeg_finish_compress(&mut cinfo);
        jpeg_destroy_compress(&mut cinfo);

        let buffer = std::slice::from_raw_parts(outbuffer, outsize as usize).to_vec();
        Image::new(buffer, img.width, img.height)
    })
    .map_err(|err| Error::Jpeg(err))
}

unsafe extern "C" fn error_exit(cinfo: &mut jpeg_common_struct) {
    let err = Box::new(
        if let Some(format_message) = cinfo.err.as_ref().and_then(|err| err.format_message) {
            let buffer = std::mem::zeroed();
            format_message(cinfo, &buffer);
            let len = buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len());
            String::from_utf8_lossy(&buffer[..len]).to_string()
        } else {
            String::from("mozjpeg failed")
        },
    );

    if !cinfo.mem.is_null() {
        if let Some(self_destruct) = (*cinfo.mem).self_destruct {
            self_destruct(cinfo);
        }
    }
    cinfo.mem = std::ptr::null_mut();
    cinfo.global_state = 0;

    std::panic::resume_unwind(err);
}

unsafe extern "C" fn output_message(_: &mut jpeg_common_struct) {
    // do nothing
}
