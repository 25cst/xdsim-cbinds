use crate::{common::*, v0::graphics::style::*};

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
