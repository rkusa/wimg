#![allow(clippy::missing_safety_doc)]

use std::mem::ManuallyDrop;

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
