#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}
