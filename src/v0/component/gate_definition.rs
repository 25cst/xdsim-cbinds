use crate::v0::{
    ComponentIdent,
    common::{Slice, Str},
    graphics::Vec2,
};

/// TODO: this will need to have a stable byte structure
/// probably need to tag repr(C) or something
/// I need to read the nomicon
#[repr(C)]
pub struct GateDefinition {
    /// The ordered input that the gate takes
    /// [ GateIOEntry ]
    pub inputs: Slice,
    /// The ordered output that the gate produces
    /// [ GateIOEntry ]
    pub outputs: Slice,

    /// The visual bounding box (dimension) of the gate
    /// The bottom left corner is (0, 0), top right corner is (width, height)
    pub bounding_box: Vec2,
}

/// Representing a single input or output connection that the gate take.
/// - name: the unique name of the input/output
/// - data_type: the type name of the input/output
/// - position: a point that is on the bounding box
#[repr(C)]
pub struct GateIOEntry {
    pub name: Str,
    pub data_type: ComponentIdent, // e.g. (package_name, gate-name, semver major, semver minor)
    pub position: Vec2,
}
