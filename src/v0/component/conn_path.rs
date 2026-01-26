use crate::common::Slice;

#[repr(C)]
pub struct ConnSegment {
    /// [ Vec2 ]
    path: Slice,
    /// if is NONE, then write NULL
    next: *const ConnJunction,
}

/// if is NONE, then write NULL
#[repr(C)]
pub struct ConnJunction {
    up: *const ConnSegment,
    right: *const ConnSegment,
    down: *const ConnSegment,
    left: *const ConnSegment,
}
