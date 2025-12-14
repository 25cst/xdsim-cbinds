use core::ffi::*;

use crate::v0::{
    app_state::{Menu, MenuInputValue},
    common::{Slice, Str},
};

pub type Properties = *const c_void;
pub type PropertiesMut = *mut c_void;

#[unsafe(no_mangle)]
pub extern "C" fn props_get_menu(props: Properties) -> Menu {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn props_get_option(props: Properties, id: *const Str) -> MenuInputValue {
    unimplemented!()
}

/// ID and value will be dropped after passing to it
/// props must not store the pointers to them
/// return NULL if no errors
/// props will be dropped after passing to it
/// slice must not store pointers to it
#[unsafe(no_mangle)]
pub extern "C" fn props_set_option(
    props: PropertiesMut,
    id: *const Str,
    value: *const MenuInputValue,
) -> *const PropertiesSetError {
    unimplemented!()
}

#[unsafe(no_mangle)]
/// returns [u8]
pub extern "C" fn props_serialize(props: Properties) -> Slice {
    unimplemented!()
}

#[unsafe(no_mangle)]
/// You must not store the pointer to the slice, the slice will be dropped
/// You must malloc for the struct manually
/// Return NULL if deserialisation failed
pub extern "C" fn props_deserialize(bytes: Slice) -> Properties {
    unimplemented!()
}

#[repr(C)]
pub enum PropertiesSetError {
    NotExist,
    Invalid(Str),
}
