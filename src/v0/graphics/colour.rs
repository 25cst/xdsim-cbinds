use crate::v0::common::Str;

/// Enum representing the colour options available for gates
#[repr(C)]
pub enum Colour {
    /// The primary colour of the gates (in a light theme, this would be black)
    Fg,

    /// The background colour (in a light theme, this would be white)
    Bg,

    /// Success colour (e.g. counter output in denary)
    Success,

    /// Info colour (e.g. counter output in denary)
    Info,

    /// Warn colour (e.g. potential incorrect operation)
    Warn,

    /// Error colour (e.g. a FSM not enabled)
    Error,

    Black,
    Blue,
    Cyan,
    Green,
    Grey,
    Magenta,
    Red,
    White,
    Yellow,
    Transparent,

    /// Arbitrary RGBA colour
    Rgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },

    /// A named colour from a palette
    ///
    /// A palette is a customisable set of colours
    /// multiple gates/connections can share the same palette
    Named {
        /// Name of the palette
        palette: Str,
        /// Name of the colour within the palette
        name: Str,
    },
}
