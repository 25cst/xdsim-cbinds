use core::ffi::*;

use crate::v0::{
    app_state::PropertiesMut,
    common::Slice,
    component::{ConnectionDefinition, ConnectionSegment, Data},
    graphics::Graphic,
};

type Connection = *const c_void;
type ConnectionMut = *mut c_void;

unsafe extern "C" {
    pub fn conn_draw(conn: Connection, request: *const ConnectionDrawRequest) -> Graphic;
    pub fn conn_def() -> *const ConnectionDefinition;
    pub fn conn_props(conn: ConnectionMut) -> PropertiesMut; // returns Properties
    pub fn conn_drop(conn: ConnectionMut);
    pub fn conn_serialize(conn: Connection) -> Slice;
    /// You must not store the pointer to the slice, the slice will be dropped
    /// You must malloc for the struct manually
    pub fn conn_deserialize(bytes: Slice) -> ConnectionMut;
}

/// Details of a request to draw a `Connection`
#[repr(C)]
pub struct ConnectionDrawRequest {
    /// Path the connection takes
    pub path: *const ConnectionSegment,
    /// Current value in the connection
    pub data: Data,
}
