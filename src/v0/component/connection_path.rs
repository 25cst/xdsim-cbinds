use crate::v0::common::Slice;

#[repr(C)]
pub struct ConnectionSegment {
    /// [ Vec2 ]
    path: Slice,
    /// if is NONE, then write NULL
    next: *const ConnectionJunction,
}

/// if is NONE, then write NULL
#[repr(C)]
pub struct ConnectionJunction {
    up: *const ConnectionSegment,
    right: *const ConnectionSegment,
    down: *const ConnectionSegment,
    left: *const ConnectionSegment,
}
