#![allow(clippy::missing_safety_doc)]

use std::os::raw::c_int;
use std::{error, fmt, mem};

pub use mozjpeg_sys::{
    boolean, jpeg_compress_struct, jpeg_decompress_struct, jpeg_error_mgr, jpeg_std_error,
    J_COLOR_SPACE,
};
use mozjpeg_sys::{c_ulong, JDIMENSION, JPEG_LIB_VERSION, JSAMPARRAY, JSAMPARRAY_MUT};

pub use mozjpeg_sys::jpeg_set_quality;

#[repr(C)]
#[must_use]
pub struct JpegResult {
    ok: bool,
    err: [u8; 200],
}

extern "C" {
    fn try_jpeg_CreateDecompress(
        dinfo: *mut jpeg_decompress_struct,
        version: c_int,
        struct_size: usize,
    ) -> JpegResult;
    pub fn try_jpeg_read_header(
        dinfo: &mut jpeg_decompress_struct,
        require_image: boolean,
    ) -> JpegResult;
    pub fn try_jpeg_start_decompress(dinfo: &mut jpeg_decompress_struct) -> JpegResult;
    pub fn try_jpeg_read_scanlines(
        cinfo: &mut jpeg_decompress_struct,
        scanlines: JSAMPARRAY_MUT,
        max_lines: JDIMENSION,
    ) -> JpegResult;
    pub fn try_jpeg_finish_decompress(dinfo: &mut jpeg_decompress_struct) -> JpegResult;
    pub fn try_jpeg_destroy_decompress(dinfo: &mut jpeg_decompress_struct) -> JpegResult;

    fn try_jpeg_CreateCompress(
        cinfo: *mut jpeg_compress_struct,
        version: c_int,
        struct_size: usize,
    ) -> JpegResult;
    pub fn try_jpeg_set_defaults(cinfo: &mut jpeg_compress_struct) -> JpegResult;
    pub fn try_jpeg_set_quality(
        cinfo: &mut jpeg_compress_struct,
        quality: c_int,
        force_baseline: boolean,
    ) -> JpegResult;
    pub fn try_jpeg_start_compress(
        dinfo: &mut jpeg_compress_struct,
        write_all_tables: boolean,
    ) -> JpegResult;
    pub fn try_jpeg_write_scanlines(
        cinfo: &mut jpeg_compress_struct,
        scanlines: JSAMPARRAY,
        num_lines: JDIMENSION,
    ) -> JpegResult;
    pub fn try_jpeg_finish_compress(cinfo: &mut jpeg_compress_struct) -> JpegResult;
    pub fn try_jpeg_destroy_compress(cinfo: &mut jpeg_compress_struct) -> JpegResult;

    pub fn try_jpeg_mem_dest(
        cinfo: &mut jpeg_compress_struct,
        outbuffer: *mut *mut u8,
        outsize: *mut c_ulong,
    ) -> JpegResult;
    pub fn try_jpeg_mem_src(
        cinfo: &mut jpeg_decompress_struct,
        inbuffer: *const u8,
        insize: c_ulong,
    ) -> JpegResult;

    #[cfg(not(target_family = "wasm"))]
    pub fn throwing_error_mgr(err: &mut jpeg_error_mgr) -> &mut jpeg_error_mgr;
}

pub unsafe fn try_jpeg_create_decompress(dinfo: *mut jpeg_decompress_struct) -> JpegResult {
    try_jpeg_CreateDecompress(
        dinfo,
        JPEG_LIB_VERSION,
        mem::size_of::<jpeg_decompress_struct>(),
    )
}

pub unsafe fn try_jpeg_create_compress(cinfo: *mut jpeg_compress_struct) -> JpegResult {
    try_jpeg_CreateCompress(
        cinfo,
        JPEG_LIB_VERSION,
        mem::size_of::<jpeg_compress_struct>(),
    )
}

#[derive(Debug)]
pub struct JpegError(String);

impl fmt::Display for JpegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl error::Error for JpegError {}

impl JpegResult {
    pub fn into_result(self) -> Result<(), JpegError> {
        if self.ok {
            Ok(())
        } else {
            let len = self
                .err
                .iter()
                .position(|c| *c == 0)
                .unwrap_or(self.err.len());

            Err(JpegError(
                String::from_utf8_lossy(&self.err[..len]).to_string(),
            ))
        }
    }
}
