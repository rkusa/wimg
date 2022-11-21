use std::mem;
use std::os::raw::{c_int, c_ulong};

use crate::error::Error;
use crate::{Image, ImageFormat};
use jpeg::*;

pub fn seed() -> u32 {
    1
}

pub fn decode(data: &[u8]) -> Result<Image, Error> {
    // println!("decode");

    // extract rotation from Exif data
    let rotation = extract_rotation(data);

    unsafe {
        let mut cinfo: jpeg_decompress_struct = std::mem::zeroed();

        let mut err: wimg_error_mgr = std::mem::zeroed();
        #[cfg(not(target_family = "wasm"))]
        {
            cinfo.common.err = throwing_error_mgr(&mut err);
        }
        #[cfg(target_family = "wasm")]
        {
            cinfo.common.err = jpeg_std_error(&mut err.r#pub);
        }

        try_jpeg_create_decompress(&mut cinfo).into_result()?;

        try_jpeg_mem_src(&mut cinfo, data.as_ptr(), data.len() as c_ulong).into_result()?;
        try_jpeg_read_header(&mut cinfo, true as boolean).into_result()?;

        // println!("width={}, height={}", cinfo.image_width, cinfo.image_height);

        cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
        try_jpeg_start_decompress(&mut cinfo).into_result()?;

        let mut width = cinfo.image_width;
        let mut height = cinfo.image_height;

        let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
        let buffer_size = row_stride * cinfo.image_height as usize;
        let mut buffer = vec![0u8; buffer_size];

        // Use the fact that the image is decoded row by row, to rotate it right away to save on
        // memory.
        match rotation {
            Some(Rotation::R90) => {
                let mut row = vec![0u8; row_stride];

                while cinfo.output_scanline < cinfo.output_height {
                    let output_scanline = cinfo.output_scanline as usize;
                    let mut jsamparray = [row.as_mut_ptr()];
                    try_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1)
                        .into_result()?;

                    // first row becomes -> last column
                    // first row + 1 becomes -> last column - 1
                    // ... and so on
                    let y_from = output_scanline;
                    let x_to = height as usize - 1 - y_from;
                    for x_from in 0..(width as usize) {
                        let y_to = x_from;
                        let from = x_from * 3;
                        let to = (x_to + y_to * height as usize) * 3;
                        buffer[to..(to + 3)].copy_from_slice(&row[from..(from + 3)]);
                    }
                }

                mem::swap(&mut width, &mut height);
            }
            Some(Rotation::R180) => {
                while cinfo.output_scanline < cinfo.output_height {
                    // start filling rows from the bottom
                    let offset =
                        (cinfo.image_height - 1 - cinfo.output_scanline) as usize * row_stride;
                    let mut jsamparray = [buffer[offset..].as_mut_ptr()];
                    try_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1)
                        .into_result()?;

                    // mirror row pixels
                    rotate180::<3>(&mut buffer[offset..offset + row_stride]);
                }
            }
            Some(Rotation::R270) => {
                let mut row = vec![0u8; row_stride];

                while cinfo.output_scanline < cinfo.output_height {
                    let output_scanline = cinfo.output_scanline as usize;
                    let mut jsamparray = [row.as_mut_ptr()];
                    try_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1)
                        .into_result()?;

                    // first row becomes -> first column starting at the bottom
                    // first row + 1 becomes -> first column + 1 starting at the bottom
                    // ... and so on
                    // but starting at the bottom, thus mirror the row
                    rotate180::<3>(&mut row[..]);

                    let y_from = output_scanline;
                    let x_to = y_from;
                    for x_from in 0..(width as usize) {
                        let y_to = x_from;
                        let from = x_from * 3;
                        let to = (x_to + y_to * height as usize) * 3;
                        buffer[to..(to + 3)].copy_from_slice(&row[from..(from + 3)]);
                    }
                }

                mem::swap(&mut width, &mut height);
            }
            None => {
                while cinfo.output_scanline < cinfo.output_height {
                    let offset = cinfo.output_scanline as usize * row_stride;
                    let mut jsamparray = [buffer[offset..].as_mut_ptr()];
                    try_jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1)
                        .into_result()?;
                }
            }
        }

        try_jpeg_finish_decompress(&mut cinfo).into_result()?;
        try_jpeg_destroy_decompress(&mut cinfo).into_result()?;

        Ok(Image::new(buffer, ImageFormat::RGB8, width, height))
    }
}

#[derive(Debug, Clone)]
pub struct EncodeOptions {
    /// 0-100 scale
    pub quality: u16,
}

pub fn encode(img: &Image, opts: &EncodeOptions) -> Result<Image, Error> {
    // println!("encode {} {}", img.width, img.height);

    let (in_color_space, input_components) = match img.format {
        ImageFormat::RGB8 => (J_COLOR_SPACE::JCS_RGB, 3),
        ImageFormat::RGBA8 => (J_COLOR_SPACE::JCS_EXT_RGBA, 4),
        _ => {
            return Err(Error::Process {
                process: "encode as JPEG",
                format: img.format,
            })
        }
    };

    unsafe {
        let mut cinfo: jpeg_compress_struct = std::mem::zeroed();

        let mut err: wimg_error_mgr = std::mem::zeroed();
        #[cfg(not(target_family = "wasm"))]
        {
            cinfo.common.err = throwing_error_mgr(&mut err);
        }
        #[cfg(target_family = "wasm")]
        {
            cinfo.common.err = jpeg_std_error(&mut err.r#pub);
        }

        try_jpeg_create_compress(&mut cinfo).into_result()?;

        let mut outsize = 0;
        let mut outbuffer = std::ptr::null_mut();
        try_jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize).into_result()?;

        cinfo.image_width = img.width;
        cinfo.image_height = img.height;
        cinfo.in_color_space = in_color_space;
        cinfo.input_components = input_components;
        try_jpeg_set_defaults(&mut cinfo).into_result()?;
        try_jpeg_set_quality(&mut cinfo, opts.quality as i32, true as c_int).into_result()?;

        try_jpeg_start_compress(&mut cinfo, true as boolean).into_result()?;

        let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
        let buffer = img.as_ref();
        while cinfo.next_scanline < cinfo.image_height {
            let offset = cinfo.next_scanline as usize * row_stride;
            let jsamparray = [buffer[offset..].as_ptr()];
            try_jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1).into_result()?;
        }

        try_jpeg_finish_compress(&mut cinfo).into_result()?;
        try_jpeg_destroy_compress(&mut cinfo).into_result()?;

        let buffer = Vec::from_raw_parts(outbuffer, outsize as usize, outsize as usize);
        Ok(Image::new(buffer, ImageFormat::JPEG, img.width, img.height))
    }
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { quality: 80 }
    }
}

#[derive(Debug)]
enum Rotation {
    R90,
    R180,
    R270,
}

fn extract_rotation(data: &[u8]) -> Option<Rotation> {
    // search exif data orientation
    // - EXIF file format: https://www.media.mit.edu/pia/Research/deepview/exif.html

    let mut offset = 2; // skip SOI (Start Of Image) marker
    while let Ok::<[u8; 4], _>(marker) = data[offset..offset + 4].try_into() {
        // markers are supposed to be prefixed with `0xFF`, if not, stop looking for Exif data
        if marker[0] != 0xFF {
            break;
        }

        // ignore any marker except the application marker 1 (used by Exif)
        let size = usize::from(u16::from_be_bytes([marker[2], marker[3]]));
        if marker[1] != 0xE1 {
            offset += size + 2;
            continue;
        }

        let app1 = &data[offset + 4..offset + 4 + (size - 2)];
        const HEADER: &[u8] = b"Exif\0\0";
        if !app1.starts_with(HEADER) {
            // not exif after all
            break;
        }

        offset += size + 2;

        let app1 = &app1[HEADER.len()..];
        let is_be = app1[0..2] == [0x4d, 0x4d];

        let h: [u8; 2] = app1[2..4].try_into().ok()?;
        let h = if is_be {
            u16::from_be_bytes(h)
        } else {
            u16::from_le_bytes(h)
        };
        if h != 0x002a {
            break;
        }

        let ifd0_offset: [u8; 4] = app1[4..8].try_into().ok()?;
        let ifd0_offset = if is_be {
            u32::from_be_bytes(ifd0_offset)
        } else {
            u32::from_le_bytes(ifd0_offset)
        } as usize;

        // parse IFD0
        let entry_count: [u8; 2] = app1[ifd0_offset..ifd0_offset + 2].try_into().ok()?;
        let entry_count = if is_be {
            u16::from_be_bytes(entry_count)
        } else {
            u16::from_le_bytes(entry_count)
        } as usize;

        // iterate tags
        for i in 0..entry_count {
            let offset = ifd0_offset + 2 + i * 12;
            let tag: [u8; 2] = app1[offset..offset + 2].try_into().ok()?;
            let tag = if is_be {
                u16::from_be_bytes(tag)
            } else {
                u16::from_le_bytes(tag)
            };
            if tag == 0x0112 {
                let offset = offset + 2 + 2 + 4;
                let val: [u8; 2] = app1[offset..offset + 2].try_into().ok()?;
                let val = if is_be {
                    u16::from_be_bytes(val)
                } else {
                    u16::from_le_bytes(val)
                };

                return match val {
                    3 => Some(Rotation::R180),
                    6 => Some(Rotation::R90),
                    8 => Some(Rotation::R270),
                    _ => None,
                };
            }
        }
    }

    None
}

/// Rotate by 180deg in place.
///
/// # Panics
///
/// Panics if `data.len()` is not a multiple of `PIXEL_SIZE`.
fn rotate180<const PIXEL_SIZE: usize>(data: &mut [u8]) {
    assert_eq!(
        data.len() % PIXEL_SIZE,
        0,
        "data length must be a multiple of the pixel size"
    );

    let pixel_count = data.len() / PIXEL_SIZE;
    for i in 0..(pixel_count / 2) {
        swap_pixel::<PIXEL_SIZE>(data, i, pixel_count - 1 - i);
    }
}

/// Swap pixels. index `b` must be greater than index `a`
fn swap_pixel<const PIXEL_SIZE: usize>(data: &mut [u8], a: usize, b: usize) {
    let p1 = a * PIXEL_SIZE;
    let p2 = b * PIXEL_SIZE;

    let (l, r) = data.split_at_mut(p2);
    l[p1..p1 + PIXEL_SIZE].swap_with_slice(&mut r[..PIXEL_SIZE]);
}
