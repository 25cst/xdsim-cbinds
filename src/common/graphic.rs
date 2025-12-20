#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}
