use crate::v0::graphics::colour::Colour;

#[repr(C)]
pub struct StrokeStyle {
    pub colour: Colour,
}

#[repr(C)]
pub struct FillStyle {
    pub colour: Colour,
}
