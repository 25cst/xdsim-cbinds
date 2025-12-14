use core::ffi::c_void;

use crate::v0::common::Slice;

pub type Data = *const c_void;

unsafe extern "C" {
    pub fn data_serialize(data: Data) -> Slice;
    /// You must not store the pointer to the slice, the slice will be dropped
    /// You must malloc for the struct manually
    pub fn data_deserialize(bytes: Slice) -> Data;
    pub fn data_drop(data: Data);
}
