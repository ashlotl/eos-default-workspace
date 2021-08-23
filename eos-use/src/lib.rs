pub mod invocations;
pub mod modules;
pub mod objekts;
pub mod utils;

#[macro_use]
extern crate mopa;

use serde::Deserialize;

pub const ALLOWED_ERRORS: u32 = 3;

#[derive(Clone, Debug, Deserialize)]
pub struct EosVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
