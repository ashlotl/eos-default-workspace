use std::{path::Path, sync::Arc};

use eos_use::modules::{EosModuleConfig, EosModuleHandle};

use crate::{DEBUG_OR_RELEASE, PLATFORM_EXTENSION};

pub fn load_synchronous(path_str: &str) -> Result<Arc<EosModuleHandle>, String> {
    let owned_path = format!(
        "target/{}/lib{}.{}",
        DEBUG_OR_RELEASE, path_str, PLATFORM_EXTENSION
    );
    let path = Path::new(owned_path.as_str());
    println!("loading path: {}", owned_path.as_str());
    if !path.exists() {
        return Err(format!(
            "Path not found: {}",
            path.to_str().unwrap_or("Path invalid.")
        ));
    }
    let path_str = match path.to_str() {
        Some(val) => val,
        None => return Err(format!("{}", "(no path)")),
    };
    unsafe {
        let lib = Arc::new(match libloading::Library::new(path_str) {
            Ok(v) => v,
            Err(e) => return Err(format!("{}", e)),
        });
        let lib_ptr = &lib as *const Arc<libloading::Library>;
        let ret = EosModuleHandle {
            config: EosModuleConfig {
                //TODO: load from file
                name: String::from("unimplemented"),
            },
            lib: lib.clone(),
            module_init: match (*lib_ptr).get(b"eos_module_init") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
            objekt_add: match (*lib_ptr).get(b"eos_objekt_add") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
            objekt_get: match (*lib_ptr).get(b"eos_objekt_get") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
            objekt_get_invocations: match (*lib_ptr).get(b"eos_objekt_get_invocations") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
            objekt_remove: match (*lib_ptr).get(b"eos_objekt_remove") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
            objekt_remove_all: match (*lib_ptr).get(b"eos_objekt_remove_all") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
            objekts_len: match (*lib_ptr).get(b"eos_objekts_len") {
                Ok(v) => v,
                Err(e) => return Err(format!("{}", e)),
            },
        };
        println!("done");
        Ok(Arc::new(ret))
    }
}
