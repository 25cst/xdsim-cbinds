#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub struct Rotation(f64);

#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub struct BoundingBox {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}
