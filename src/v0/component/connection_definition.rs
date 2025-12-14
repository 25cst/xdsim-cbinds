use crate::v0::PackageIdent;

/// Information for a Connection
#[repr(C)]
pub struct ConnectionDefinition {
    /// Definition schema version number
    pub version: u32,

    /// Data type the connection carries
    pub data_type: PackageIdent, // e.g. (package_name, semver major, semver minor)

    /// Connection identifier: the unique identifier for the connection type
    /// filled in by macro
    pub identifier: PackageIdent, // e.g. (package_name, semver major, semver minor)
}
