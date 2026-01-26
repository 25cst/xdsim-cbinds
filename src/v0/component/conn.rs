use core::ffi::*;

use crate::{
    common::Slice,
    v0::{
        app_state::PropertiesMut,
        component::{ConnDefinition, ConnSegment, Data},
        graphics::Graphic,
    },
};

pub type Conn = *const c_void;
pub type ConnMut = *mut c_void;

unsafe extern "C" {
    pub fn conn_draw(conn: Conn, path: *const ConnSegment, data: Data) -> Graphic;

    pub fn conn_def(conn: Conn) -> ConnDefinition;

    pub fn conn_props(conn: ConnMut) -> PropertiesMut;

    pub fn conn_drop(conn: ConnMut);

    pub fn conn_serialize(conn: Conn) -> Slice;

    /// You must not store the pointer to the slice, the slice will be dropped
    /// You must malloc for the struct manually
    pub fn conn_deserialize(bytes: *const Slice) -> ConnMut;

    pub fn conn_default() -> ConnMut;
}
