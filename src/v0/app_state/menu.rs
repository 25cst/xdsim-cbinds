use crate::v0::common::*;

/// A menu is a list of MenuItems
#[repr(C)]
pub struct Menu {
    /// [ MenuItem ]
    pub items: Slice,
}

#[repr(C)]
pub struct MenuItem {
    pub tooltip: Str,
    pub item_type: MenuItemVariant,
}

#[repr(C)]
pub enum MenuItemVariant {
    /// Can be fold/unfold to hide/reveal menu items
    Foldable {
        title: Str,
        /// [ MenuItem ]
        items: Slice,
    },
    Text {
        content: Str,
    },
    Input {
        id: Str,
        input_type: MenuInput,
    },
}

#[repr(C)]
pub enum MenuInput {
    String {
        style: MenuInputStringStyle,
    },
    Integer {
        min: i64,
        max: i64,
        style: MenuInputIntegerStyle,
    },
    Float {
        min: f64,
        max: f64,
        style: MenuInputFloatStyle,
    },
    Boolean {
        style: MenuInputBooleanStyle,
    },
}

#[repr(C)]
pub enum MenuInputStringStyle {
    Inline {
        placeholder: Str,
    },
    Multiline {
        max_lines: u32,
        placeholder: Str,
    },
    Dropdown {
        /// [ Str ]
        options: Slice,
    },
}

#[repr(C)]
pub enum MenuInputIntegerStyle {
    Slider,
    TextBox,
    SliderAndTextBox,
}

#[repr(C)]
pub enum MenuInputFloatStyle {
    Slider,
    TextBox,
    SliderAndTextBox,
}

#[repr(C)]
pub enum MenuInputBooleanStyle {
    CheckBox,
}

#[repr(C)]
pub enum MenuInputValue {
    String(Str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
