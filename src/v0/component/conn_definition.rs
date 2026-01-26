use super::ComponentIdent;

/// Information for a Conn
#[repr(C)]
pub struct ConnDefinition {
    /// Data type the conn carries
    pub data_type: ComponentIdent, // e.g. (package_name, semver major, semver minor)
}
