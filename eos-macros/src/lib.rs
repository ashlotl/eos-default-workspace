use std::fs;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

fn cfg_file_load(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(contents.as_str()).unwrap()
}

#[proc_macro]
pub fn cfg_file_load_strs(ast: TokenStream) -> TokenStream {
    let path: LitStr = parse_macro_input!(ast);
    let module_paths = cfg_file_load(path.value());
    let mut ret = String::from(
        "
        let mut macroRet = vec![];
    ",
    );
    for module_path in module_paths {
        ret.push_str(
            format!(
                "
            macroRet.push(\"{}\");
        ",
                module_path.as_str(),
            )
            .as_str(),
        );
    }
    ret.parse().unwrap()
}

#[proc_macro]
pub fn cfg_file_load_idents(ast: TokenStream) -> TokenStream {
    let path: LitStr = parse_macro_input!(ast);
    let module_paths = cfg_file_load(path.value());
    let mut string_ret = String::from(
        "
        #[allow(incorrect_ident_case)]
        let mut macroRet = std::collections::BTreeMap::new();
    ",
    );
    for module_path in module_paths {
        let s = module_path.as_str();
        string_ret.push_str(
            format!(
                "
            macroRet.insert(
                String::from(\"{}\"), std::sync::Arc::new(
                    eos_use::modules::EosModuleHandle::new(
                        {}::eos_module_init,
                        {}::eos_objekt_add,
                        {}::eos_objekt_get,
                        {}::eos_objekt_get_invocations,
                        {}::eos_objekt_remove,
                        {}::eos_objekt_remove_all,
                        {}::eos_objekts_len,
                        {}::eos_objekts_get_keys,
                    )
                )
            );
        ",
                module_path, s, s, s, s, s, s, s, s,
            )
            .as_str(),
        );
    }
    string_ret.parse().unwrap()
}

#[proc_macro]
pub fn default_module_glue(_ast: TokenStream) -> TokenStream {
    (quote! {
mod module_impl;

use std::{collections::BTreeMap, fs, sync::Arc};

use eos_use::{
    invocations::invocation::InvocationTemplateInformation,
    modules::EosModuleHandle,
    objekts::{EosModuleObjekt, EosObjekt},
};

use parking_lot::RwLock;

static mut CREATED_OBJEKTS: Option<
    Arc<RwLock<BTreeMap<String, Arc<RwLock<module_impl::EosObjektType>>>>>,
> = None;
static mut MODULE_LIST: Option<Arc<RwLock<BTreeMap<String, Arc<EosModuleHandle>>>>> = None;

#[no_mangle]
pub fn eos_module_init(module_list: Arc<RwLock<BTreeMap<String, Arc<EosModuleHandle>>>>) {
    unsafe {
        //TODO: runtime file loads shouldn't be possible if using static modules in order to allow easy creation of singular binaries
        let path_string = format!("{}/objekt_list.eos.ron", env!("CARGO_MANIFEST_DIR"));
        let path = path_string.as_str();
        let contents = fs::read_to_string(path).expect(format!("{} was not found.", path).as_str());
        let mut store: BTreeMap<String, module_impl::EosObjektType> =
            ron::from_str(&*(contents.as_str() as *const str))
                .expect(format!("File {} could not be parsed", path).as_str());
        let mut locked_store = BTreeMap::new();
        let key_iter: Vec<String> = store.keys().map(|e| e.clone()).collect();
        for key in key_iter {
            let entry = (key.clone(), store.remove(&key).unwrap());
            locked_store.insert(entry.0, Arc::new(RwLock::new(entry.1)));
        }
        CREATED_OBJEKTS = Some(Arc::new(RwLock::new(locked_store)));
        println!("{:#?}", CREATED_OBJEKTS);
        MODULE_LIST = Some(module_list);
    }
}

#[no_mangle]
pub fn eos_objekt_add(name: String, deserialize_from: String) {
    let ret = module_impl::EosObjektType::create_objekt(deserialize_from).unwrap();
    unsafe {
        let mut created_objects = CREATED_OBJEKTS.as_mut().unwrap().write();
        created_objects.insert(name, Arc::new(RwLock::new(ret)));
    }
}

#[no_mangle]
pub fn eos_objekt_get(name: &String) -> Arc<RwLock<dyn EosObjekt>> {
    unsafe {
        let created_objects = CREATED_OBJEKTS.as_ref().unwrap();
        created_objects.read()[name].clone()
    }
}

#[no_mangle]
pub fn eos_objekt_get_invocations(objekt_name: String) -> InvocationTemplateInformation {
    let objekt = eos_objekt_get(&objekt_name);
    let guard = objekt.read();
    guard.get_invocations(objekt_name, objekt_getter)
}

#[no_mangle]
pub fn eos_objekt_remove(name: &String) {
    unsafe {
        let mut created_objekts = CREATED_OBJEKTS.as_mut().unwrap().write();
        created_objekts.remove(name);
    }
}

#[no_mangle]
pub fn eos_objekt_remove_all() {
    unsafe {
        let mut created_objekts = CREATED_OBJEKTS.as_mut().unwrap().write();
        *created_objekts = BTreeMap::new();
    }
}

#[no_mangle]
pub fn eos_objekts_len() -> usize {
    unsafe { CREATED_OBJEKTS.as_mut().unwrap().read().len() }
}

#[no_mangle]
pub fn eos_objekts_get_keys() -> Vec<String> {
    unsafe {
        let created_objekts = CREATED_OBJEKTS.as_ref().unwrap().read();
        let keys = created_objekts.keys();
        let mut ret = vec![];
        for key in keys {
            ret.push(key.clone());
        }
        ret
    }
}

fn objekt_getter(objekt_name: String) -> Result<Arc<RwLock<dyn EosObjekt>>, String> {
    unsafe {
        if let Some(v) = CREATED_OBJEKTS.as_ref().unwrap().read().get(&objekt_name) {
            return Ok(v.clone());
        } else {
            return Err(format!(
                "Invocation could not find objekt with name: {}",
                objekt_name,
            ));
        }
    }
}

    }).into()
}
