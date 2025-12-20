use core::ffi::*;

use crate::{
    common::Slice,
    v0::{
        app_state::PropertiesMut,
        component::GateDefinition,
        graphics::{Direction, Graphic, Vec2},
    },
};

pub type Gate = *const c_void;
pub type GateMut = *mut c_void;

unsafe extern "C" {
    /// returns [ Data ]
    /// individual Data must be malloced
    /// it will be freed by the program
    /// in this particular case, slice.drop should only drop the slice
    /// and not the individual Data
    pub fn gate_tick(gate: GateMut, request: *const GateTickRequest) -> Slice;

    pub fn gate_draw(gate: Gate, request: *const GateDrawRequest) -> Graphic;

    pub fn gate_def(gate: Gate) -> GateDefinition;

    /// Return NULL if no properties
    pub fn gate_props(gate: GateMut) -> PropertiesMut;

    pub fn gate_serialize(gate: Gate) -> Slice;

    pub fn gate_deserialize(bytes: *const Slice) -> GateMut;

    pub fn gate_default() -> GateMut;

    pub fn gate_drop(conn: GateMut);
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
