use core::ffi::*;

use crate::v0::{
    app_state::PropertiesMut,
    common::Slice,
    component::GateDefinition,
    graphics::{Direction, Graphic, Vec2},
};

pub type Gate = *const c_void;
pub type GateMut = *mut c_void;

/// returns [ Data ]
/// individual Data must be malloced
/// it will be freed by the program
/// in this particular case, slice.drop should only drop the slice
/// and not the individual Data
#[unsafe(no_mangle)]
pub extern "C" fn gate_tick(gate: GateMut, request: *const GateTickRequest) -> Slice {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn gate_draw(gate: Gate, request: *const GateDrawRequest) -> Graphic {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn gate_def(gate: Gate) -> GateDefinition {
    unimplemented!()
}

/// Return NULL if no properties
#[unsafe(no_mangle)]
pub extern "C" fn gate_props(gate: GateMut) -> PropertiesMut {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn gate_serialize(gate: Gate) -> Slice {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn gate_deserialize(bytes: Slice) -> GateMut {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn gate_default() -> GateMut {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn gate_drop(conn: GateMut) {
    unimplemented!()
}

/// A single gate tick request
#[repr(C)]
pub struct GateTickRequest {
    /// Inputs to the gate
    /// [ *const Data ]
    pub inputs: Slice,
}

/// A single gate draw request
#[repr(C)]
pub struct GateDrawRequest {
    /// One of the four the gate is facing (rotation)
    pub direction: Direction,
    /// The size of the bounding box previously provided
    pub dimension: Vec2,
}
