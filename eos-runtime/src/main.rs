mod active_modules;

#[macro_use]
extern crate eos_macros;

use futures::executor::block_on;

#[cfg(debug_assertions)]
pub const DEBUG_OR_RELEASE: &'static str = "debug";

#[cfg(not(debug_assertions))]
pub const DEBUG_OR_RELEASE: &'static str = "release";

//TODO: figuring out dynamic library types of more os'es might not kill you
#[cfg(target_os = "linux")]
pub const PLATFORM_EXTENSION: &'static str = "so";

#[cfg(target_os = "macos")]
pub const PLATFORM_EXTENSION: &'static str = "dylib";

#[cfg(target_os = "windows")]
pub const PLATFORM_EXTENSION: &'static str = "dll";

fn main() {
    println!("loading modules");
    block_on(active_modules::load_modules().unwrap());
}
