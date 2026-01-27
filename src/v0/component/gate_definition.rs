use super::ComponentIdent;

use crate::common::*;

/// TODO: this will need to have a stable byte structure
/// probably need to tag repr(C) or something
/// I need to read the nomicon
#[repr(C)]
pub struct GateDefinition {
    /// The ordered input that the gate takes
    /// [ GateInputEntry ]
    pub inputs: Slice,
    /// The ordered output that the gate produces
    /// [ GateOutputEntry ]
    pub outputs: Slice,

    /// The visual bounding box (dimension) of the gate
    /// The bottom left corner is (0, 0), top right corner is (width, height)
    pub bounding_box: Vec2,
}

/// Representing a single input connection that the gate take.
/// - name: the unique name of the input/output
/// - data_type: the type name of the input/output
/// - direction: the side of the bounding box the connection is on
/// - position: length of the side of the bounding box to the left (counter clockwise direction) of the socket
#[repr(C)]
pub struct GateInputEntry {
    pub name: Str,
    pub data_type_req: ComponentIdent, // e.g. (package_name, gate-name, semver major, semver minor)
    pub direction: Direction,
    pub position: f64,
}

// they are the same layout for now, but can change in the future
/// Representing a single output connection that the gate take.
/// - name: the unique name of the input/output
/// - data_type: the type name of the input/output
/// - direction: the side of the bounding box the connection is on
/// - position: length of the side of the bounding box to the left (counter clockwise direction) of the socket
#[repr(C)]
pub struct GateOutputEntry {
    pub name: Str,
    pub data_type: ComponentIdent, // e.g. (package_name, gate-name, semver major, semver minor)
    pub direction: Direction,
    pub position: f64,
}
