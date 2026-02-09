#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

#[repr(C)]
#[cfg_attr(feature = "impl", derive(Clone, Copy))]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}
