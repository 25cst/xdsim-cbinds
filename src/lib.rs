#![no_std]
#![allow(unused_variables)]

#[cfg(any(feature = "v0-conn", feature = "v0-data", feature = "v0-gate"))]
pub mod v0;

mod version;
