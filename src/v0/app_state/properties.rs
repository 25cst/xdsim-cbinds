use core::ffi::*;

use crate::v0::{
    app_state::{Menu, MenuInputValue},
    common::{Slice, Str},
};

pub type Properties = *const c_void;
pub type PropertiesMut = *mut c_void;

unsafe extern "C" {
    pub fn props_get_menu(props: Properties) -> Menu;
    pub fn props_get_option(props: Properties, id: *const Str) -> MenuInputValue;
    /// ID and value will be dropped after passing to it
    /// props must not store the pointers to them
    /// return NULL if no errors
    pub fn props_set_option(
        props: PropertiesMut,
        id: *const Str,
        value: *const MenuInputValue,
    ) -> *const PropertiesSetError;
    /// props will be dropped after passing to it
    /// slice must not store pointers to it
    pub fn props_serialize(props: Properties) -> Slice; // [u8]
    /// You must not store the pointer to the slice, the slice will be dropped
    /// You must malloc for the struct manually
    /// Return NULL if deserialisation failed
    pub fn props_deserialize(bytes: Slice) -> Properties; // container
}

#[repr(C)]
pub enum PropertiesSetError {
    NotExist,
    Invalid(Str),
}
