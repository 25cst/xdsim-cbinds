use crate::v0::ComponentIdent;

/// Information for a Connection
#[repr(C)]
pub struct ConnectionDefinition {
    /// Definition schema version number
    pub version: u32,

    /// Data type the connection carries
    pub data_type: ComponentIdent, // e.g. (package_name, semver major, semver minor)
}
