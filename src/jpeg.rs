use std::mem;

use crate::error::Error;
use crate::{Image, ImageFormat};
use mozjpeg_sys::*;

pub fn seed() -> u32 {
    0
}

pub fn decode(data: &[u8]) -> Result<Image, Error> {
    // println!("decode");

    // extract rotation from Exif data
    let rotation = extract_rotation(data);

    std::panic::catch_unwind(|| unsafe {
        let mut cinfo: jpeg_decompress_struct = std::mem::zeroed();

        let mut err: jpeg_error_mgr = std::mem::zeroed();
        jpeg_std_error(&mut err);
        err.error_exit = Some(error_exit);
        err.output_message = Some(output_message);
        cinfo.common.err = &mut err;

        jpeg_create_decompress(&mut cinfo);

        jpeg_mem_src(&mut cinfo, data.as_ptr(), data.len() as c_ulong);
        jpeg_read_header(&mut cinfo, true as boolean);

        // println!("width={}, height={}", cinfo.image_width, cinfo.image_height);

        cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
        jpeg_start_decompress(&mut cinfo);

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
                    jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);

                    // first row becomes -> last column
                    // first row + 1 becomes -> last column - 1
                    // ... and so on
                    let y_from = output_scanline;
                    let x_to = height as usize - 1 - y_from;
                    for x_from in 0..(width as usize) {
                        let y_to = x_from;
                        let from = x_from * 3;
                        let to = (x_to + y_to * height as usize) * 3;
                        (&mut buffer[to..(to + 3)]).copy_from_slice(&row[from..(from + 3)]);
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
                    jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);

                    // mirror row pixels
                    rotate180::<3>(&mut buffer[offset..offset + row_stride]);
                }
            }
            Some(Rotation::R270) => {
                let mut row = vec![0u8; row_stride];

                while cinfo.output_scanline < cinfo.output_height {
                    let output_scanline = cinfo.output_scanline as usize;
                    let mut jsamparray = [row.as_mut_ptr()];
                    jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);

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
                        (&mut buffer[to..(to + 3)]).copy_from_slice(&row[from..(from + 3)]);
                    }
                }

                mem::swap(&mut width, &mut height);
            }
            None => {
                while cinfo.output_scanline < cinfo.output_height {
                    let offset = cinfo.output_scanline as usize * row_stride;
                    let mut jsamparray = [buffer[offset..].as_mut_ptr()];
                    jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);
                }
            }
        }

        jpeg_finish_decompress(&mut cinfo);
        jpeg_destroy_decompress(&mut cinfo);

        Image::new(buffer, ImageFormat::RGB8, width, height)
    })
    .map_err(|err| Error::Jpeg(err.downcast::<String>().unwrap_or_default()))
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
        cinfo.in_color_space = in_color_space;
        cinfo.input_components = input_components;
        jpeg_set_defaults(&mut cinfo);
        jpeg_set_quality(&mut cinfo, opts.quality as i32, true as boolean);

        jpeg_start_compress(&mut cinfo, true as boolean);

        let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
        let buffer = img.as_ref();
        while cinfo.next_scanline < cinfo.image_height {
            let offset = cinfo.next_scanline as usize * row_stride;
            let jsamparray = [buffer[offset..].as_ptr()];
            jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1);
        }

        jpeg_finish_compress(&mut cinfo);
        jpeg_destroy_compress(&mut cinfo);

        let buffer = Vec::from_raw_parts(outbuffer, outsize as usize, outsize as usize);
        Image::new(buffer, ImageFormat::JPEG, img.width, img.height)
    })
    .map_err(|err| Error::Jpeg(err.downcast::<String>().unwrap_or_default()))
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self { quality: 80 }
    }
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

    // jpeg_destroy
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
        let size = usize::from(u16::from_be_bytes([marker[2], marker[3]])) - 2;
        if marker[1] != 0xE1 {
            offset += size;
            continue;
        }

        let app1 = &data[offset + 4..offset + 4 + size];
        const HEADER: &[u8] = b"Exif\0\0";
        if !app1.starts_with(HEADER) {
            // not exif after all
            break;
        }

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
