use core::ffi::*;

use crate::v0::{
    app_state::PropertiesMut,
    common::Slice,
    component::GateDefinition,
    graphics::{Direction, Graphic, Vec2},
};

pub type Gate = *const c_void;
pub type GateMut = *mut c_void;

unsafe extern "C" {
    pub fn gate_tick(gate: GateMut, request: *const GateTickRequest) -> Slice; // [ *const Data ]
    pub fn gate_draw(gate: Gate, request: *const GateDrawRequest) -> Graphic;
    pub fn gate_def(gate: Gate) -> GateDefinition;
    pub fn gate_props(gate: GateMut) -> PropertiesMut;
    pub fn gate_serialize(gate: Gate) -> Slice;
    pub fn gate_deserialize(bytes: Slice) -> GateMut;
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
