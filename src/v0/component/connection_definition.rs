use crate::v0::ComponentIdent;

/// Information for a Connection
#[repr(C)]
pub struct ConnectionDefinition {
    /// Data type the connection carries
    pub data_type: ComponentIdent, // e.g. (package_name, semver major, semver minor)
}
