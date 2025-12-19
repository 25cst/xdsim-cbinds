use crate::{
    common::Slice,
    v0::graphics::{Vec2, style::*},
};

#[repr(C)]
pub enum Element {
    Line {
        /// [ Vec2 ]
        points: Slice,
        stroke: StrokeStyle,
    },
    Rect {
        pos: Vec2,
        size: Vec2,
        stroke: StrokeStyle,
        fill: FillStyle,
    },
}
