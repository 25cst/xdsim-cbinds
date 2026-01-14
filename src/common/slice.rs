use core::ffi::*;

/// A non-resizeable array with length
#[repr(C)]
pub struct Slice {
    pub first: *mut c_void,
    pub length: u64,
    pub item_size: u64,
    pub drop: extern "C" fn(first: *mut c_void, item_size: u64, length: u64),
}

/// A non-resizeable, null-terminated string
#[repr(C)]
pub struct Str {
    pub first: *mut c_char,
    pub drop: extern "C" fn(first: *mut c_char),
}

#[cfg(feature = "impl")]
impl Drop for Slice {
    fn drop(&mut self) {
        (self.drop)(self.first, self.item_size, self.length);
    }
}

#[cfg(feature = "impl")]
impl Drop for Str {
    fn drop(&mut self) {
        (self.drop)(self.first);
    }
}
