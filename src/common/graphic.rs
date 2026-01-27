#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}
