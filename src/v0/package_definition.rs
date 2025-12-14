use crate::v0::common::{Slice, Str};

#[repr(C)]
pub struct PackageDefinition {
    pub ident: PackageIdent, // package name, semver major, semver minor, semver patch
    pub component_type: ComponentType,

    /// [ Str ]
    pub authors: Slice,
    pub description: Str,
    pub homepage: Str,
}

#[repr(C)]
pub enum ComponentType {
    #[cfg(feature = "v0-gate")]
    Gate,
    #[cfg(feature = "v0-conn")]
    Connection,
    #[cfg(feature = "v0-data")]
    Data,
}

#[repr(C)]
pub struct PackageIdent {
    pub name: Str,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[unsafe(no_mangle)]
pub extern "C" fn package_definition() -> PackageDefinition {
    unimplemented!()
}
