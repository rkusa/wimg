#![allow(clippy::missing_safety_doc)]

pub mod error;
mod ffi;
mod jpeg;
mod resize;

use std::mem::ManuallyDrop;
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug)]
pub struct Image {
    ptr: NonNull<u8>,
    len: usize,
    cap: usize,
    width: u32,
    height: u32,
}

impl Image {
    fn new(v: Vec<u8>, width: u32, height: u32) -> Self {
        let mut v = ManuallyDrop::new(v);
        Image {
            ptr: unsafe { NonNull::new_unchecked(v.as_mut_ptr()) },
            len: v.len(),
            cap: v.capacity(),
            width,
            height,
        }
    }

    fn into_raw(self) -> *mut Image {
        Box::into_raw(Box::new(self))
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
