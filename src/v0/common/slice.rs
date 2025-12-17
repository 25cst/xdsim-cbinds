use core::ffi::*;

/// A non-resizeable array with length
#[repr(C)]
pub struct Slice {
    pub first: *const c_void,
    pub length: u64,
    pub drop: extern "C" fn(*mut c_void, u64),
}

/// A non-resizeable, null-terminated string
#[repr(C)]
pub struct Str {
    pub first: *const c_char,
    pub drop: extern "C" fn(*mut c_char),
}
