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
    /// inputs is an array of Data
    pub fn gate_tick(gate: GateMut, inputs: Slice) -> Slice;

    /// direction: one of the four the gate is facing (rotation)
    /// dimension: the size of the bounding box previously provided
    pub fn gate_draw(gate: Gate, direction: Direction, bounding_box: Vec2) -> Graphic;

    pub fn gate_def(gate: Gate) -> GateDefinition;

    /// Return NULL if no properties
    pub fn gate_props(gate: GateMut) -> PropertiesMut;

    pub fn gate_serialize(gate: Gate) -> Slice;

    pub fn gate_deserialize(bytes: *const Slice) -> GateMut;

    pub fn gate_default() -> GateMut;

    pub fn gate_drop(conn: GateMut);
}
