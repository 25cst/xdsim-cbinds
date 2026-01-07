use crate::common::Str;

#[repr(C)]
pub struct ComponentIdent {
    pub package: Str,
    pub version: Str,
    pub component: Str,
}
