use core::ffi::*;

/// A non-resizeable array with length
#[repr(C)]
pub struct Slice {
    first: *const c_void,
    length: u64,
    drop: extern "C" fn(*const c_void, u64),
}

/// A non-resizeable, null-terminated string
#[repr(C)]
pub struct Str {
    first: *const c_char,
    drop: extern "C" fn(*const c_char),
}
