use core::ffi::c_void;

use crate::v0::common::Slice;

pub type Data = *const c_void;

#[unsafe(no_mangle)]
#[cfg(feature = "v0-data")]
pub extern "C" fn data_serialize(data: Data) -> Slice {
    unimplemented!()
}

/// You must not store the pointer to the slice, the slice will be dropped
/// You must malloc for the struct manually
#[unsafe(no_mangle)]
#[cfg(feature = "v0-data")]
pub extern "C" fn data_deserialize(bytes: Slice) -> Data {
    unimplemented!()
}

#[unsafe(no_mangle)]
#[cfg(feature = "v0-data")]
pub extern "C" fn data_drop(data: Data) {
    unimplemented!()
}
