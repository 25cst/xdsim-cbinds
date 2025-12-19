use crate::common::Str;

#[repr(C)]
pub struct ComponentIdent {
    pub package: Str,
    pub component: Str,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}
