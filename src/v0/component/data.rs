use core::ffi::c_void;

use crate::common::Slice;

pub type Data = *const c_void;
pub type DataMut = *mut c_void;

#[cfg(feature = "v0-data")]
unsafe extern "C" {
    pub fn data_serialize(data: Data) -> Slice;

    /// You must not store the pointer to the slice, the slice will be dropped
    /// You must malloc for the struct manually
    pub fn data_deserialize(bytes: *const Slice) -> Data;

    pub fn data_drop(data: DataMut);

    pub fn data_default() -> Data;
}
