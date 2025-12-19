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

#[unsafe(no_mangle)]
pub extern "C" fn conn_draw(conn: Connection, request: *const ConnectionDrawRequest) -> Graphic {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn conn_def(conn: Connection) -> *const ConnectionDefinition {
    unimplemented!()
}

/// Return NULL if no properties
#[unsafe(no_mangle)]
pub extern "C" fn conn_props(conn: ConnectionMut) -> PropertiesMut {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn conn_drop(conn: ConnectionMut) {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn conn_serialize(conn: Connection) -> Slice {
    unimplemented!()
}

/// You must not store the pointer to the slice, the slice will be dropped
/// You must malloc for the struct manually
#[unsafe(no_mangle)]
pub extern "C" fn conn_deserialize(bytes: *const Slice) -> ConnectionMut {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn conn_default() -> ConnectionMut {
    unimplemented!()
}

/// Details of a request to draw a `Connection`
#[repr(C)]
pub struct ConnectionDrawRequest {
    /// Path the connection takes
    pub path: *const ConnectionSegment,
    /// Current value in the connection
    pub data: Data,
}
