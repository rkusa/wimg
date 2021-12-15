#![allow(clippy::missing_safety_doc)]

mod crop;
pub mod error;
mod ffi;
pub mod jpeg;
pub mod resize;

use std::fmt::Display;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

#[repr(C)]
#[repr(C)]
#[derive(Debug)]
pub struct Image {
    ptr: NonNull<u8>,
    len: usize,
    cap: usize,
    format: ImageFormat,
    width: u32,
    height: u32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    RGB8 = 1,
    RGBA8 = 2,
    JPEG = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum PixelFormat {
    RGB8 = 1,
    RGBA8 = 2,
}

impl Image {
    fn new(v: Vec<u8>, format: ImageFormat, width: u32, height: u32) -> Self {
        let mut v = ManuallyDrop::new(v);
        Self {
            ptr: unsafe { NonNull::new_unchecked(v.as_mut_ptr()) },
            len: v.len(),
            cap: v.capacity(),
            format,
            width,
            height,
        }
    }

    fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }
}

impl PixelFormat {
    pub const fn pixel_size(&self) -> usize {
        match self {
            PixelFormat::RGB8 => 3,
            PixelFormat::RGBA8 => 4,
        }
    }
}

impl AsRef<[u8]> for Image {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.ptr.as_ptr(), self.len as usize, self.cap as usize);
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image::new(self.as_ref().to_vec(), self.format, self.width, self.height)
    }
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ImageFormat::RGB8 => "RGB8",
            ImageFormat::RGBA8 => "RGBA8",
            ImageFormat::JPEG => "JPEG",
        })
    }
}
