use crate::common::Slice;

#[repr(C)]
pub struct ConnSegment {
    /// [ Vec2 ]
    pub path: Slice,
    /// if is NONE, then write NULL
    pub next: *const ConnJunction,
}

/// if is NONE, then write NULL
#[repr(C)]
pub struct ConnJunction {
    pub up: *const ConnSegment,
    pub right: *const ConnSegment,
    pub down: *const ConnSegment,
    pub left: *const ConnSegment,
}
