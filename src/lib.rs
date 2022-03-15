#![allow(clippy::missing_safety_doc)]

pub mod avif;
mod crop;
pub mod error;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod hash;
pub mod jpeg;
pub mod png;
pub mod resize;
pub mod webp;

use std::fmt::Display;
#[cfg(feature = "ffi")]
use std::ptr::NonNull;

#[cfg(feature = "ffi")]
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

#[cfg(not(feature = "ffi"))]
#[derive(Debug)]
pub struct Image {
    data: Vec<u8>,
    format: ImageFormat,
    width: u32,
    height: u32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    RGB8 = 1,
    RGBA8,
    JPEG,
    PNG,
    AVIF,
    WEBP,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum PixelFormat {
    RGB8 = 1,
    RGBA8 = 2,
}

impl Image {
    #[cfg(feature = "ffi")]
    fn new(v: Vec<u8>, format: ImageFormat, width: u32, height: u32) -> Self {
        let mut v = std::mem::ManuallyDrop::new(v);
        Self {
            ptr: unsafe { NonNull::new_unchecked(v.as_mut_ptr()) },
            len: v.len(),
            cap: v.capacity(),
            format,
            width,
            height,
        }
    }

    #[cfg(feature = "ffi")]
    fn into_raw(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    #[cfg(not(feature = "ffi"))]
    fn new(data: Vec<u8>, format: ImageFormat, width: u32, height: u32) -> Self {
        Self {
            data,
            format,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn mime_type(&self) -> &'static str {
        match self.format {
            ImageFormat::RGB8 | ImageFormat::RGBA8 => "application/octet-stream",
            ImageFormat::JPEG => "image/jpeg",
            ImageFormat::PNG => "image/png",
            ImageFormat::AVIF => "image/avif",
            ImageFormat::WEBP => "image/webp",
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        #[cfg(feature = "ffi")]
        unsafe {
            Vec::from_raw_parts(self.ptr.as_ptr(), self.len as usize, self.cap as usize)
        }
        #[cfg(not(feature = "ffi"))]
        self.data
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
        #[cfg(feature = "ffi")]
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
        #[cfg(not(feature = "ffi"))]
        &self.data
    }
}

impl AsMut<[u8]> for Image {
    fn as_mut(&mut self) -> &mut [u8] {
        #[cfg(feature = "ffi")]
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_mut(), self.len)
        }
        #[cfg(not(feature = "ffi"))]
        &mut self.data
    }
}

#[cfg(feature = "ffi")]
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
            ImageFormat::PNG => "PNG",
            ImageFormat::AVIF => "AVIF",
            ImageFormat::WEBP => "WEBP",
        })
    }
}
