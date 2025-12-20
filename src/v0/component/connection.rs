use core::ffi::*;

use crate::{
    common::Slice,
    v0::{
        app_state::PropertiesMut,
        component::{ConnectionDefinition, ConnectionSegment, Data},
        graphics::Graphic,
    },
};

pub type Connection = *const c_void;
pub type ConnectionMut = *mut c_void;

unsafe extern "C" {
    pub fn conn_draw(conn: Connection, path: *const ConnectionSegment, data: Data) -> Graphic;

    pub fn conn_def(conn: Connection) -> ConnectionDefinition;

    pub fn conn_props(conn: ConnectionMut) -> PropertiesMut;

    pub fn conn_drop(conn: ConnectionMut);

    pub fn conn_serialize(conn: Connection) -> Slice;

    /// You must not store the pointer to the slice, the slice will be dropped
    /// You must malloc for the struct manually
    pub fn conn_deserialize(bytes: *const Slice) -> ConnectionMut;

    pub fn conn_default() -> ConnectionMut;
}
