use crate::v0::common::Slice;

// TODO: perhaps use a builder style way to create a graphic, like
// Graphic::default().line(...).circle(...)
// so we can compute the bounding box of the graphic
// which could be useful later on
#[repr(C)]
pub struct Graphic {
    /// [ Element ]
    pub elements: Slice,
}
