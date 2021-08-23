use std::{collections::BTreeMap, future::Future, sync::Arc};

use parking_lot::RwLock;

use eos_use::{invocations::invoker::Invoker, modules::EosModuleHandle};

#[cfg(not(feature = "use_static_modules"))]
mod dynamic_module_creation;

pub fn load_modules() -> Result<impl Future, String> {
    #[cfg(not(feature = "use_static_modules"))]
    let modules: Arc<RwLock<BTreeMap<String, Arc<EosModuleHandle>>>> = Arc::new(RwLock::new({
        println!("using dynamic modules");
        cfg_file_load_strs!("module_list.eos.json");
        let mut map = BTreeMap::new();
        for path in macroRet {
            map.insert(
                String::from(path),
                dynamic_module_creation::load_synchronous(path)?,
            );
        }
        map
    }));

    #[cfg(feature = "use_static_modules")]
    let modules: Arc<RwLock<BTreeMap<String, Arc<EosModuleHandle>>>> = Arc::new(RwLock::new({
        println!("using static modules");

        cfg_file_load_idents!("module_list.eos.json");
        macroRet
    }));

    for module in &*modules.read() {
        println!("calling module init on module: {}", module.0);
        let module_list_ref = modules.clone();
        module.1.call_module_init(module_list_ref)?;
        println!("done");
    }

    let mut invocation_list = vec![];
    let mut entrypoint_list = vec![];

    for module in &*modules.read() {
        println!("adding objekts from module: {}", module.0);
        let len = module.1.call_objekts_len()?;
        for i in 0..len {
            let mut info = module.1.call_objekt_get_invocations(i)?;
            entrypoint_list.append(&mut info.entrypoint_list);
            invocation_list.append(&mut info.invocation_list);
        }
    }

    let mut invoker = Invoker::new();
    invoker.push_invocations(invocation_list);
    invoker.push_entrypoints(entrypoint_list);
    let built_invoker = invoker.check_for_errors_and_build()?;

    Ok(built_invoker.run())
}
