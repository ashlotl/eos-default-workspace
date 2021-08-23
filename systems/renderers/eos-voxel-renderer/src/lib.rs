mod module_impl;

use std::{collections::BTreeMap, fs, sync::Arc};

use eos_use::{
    invocations::{
        invocation::{InvocationTemplate, InvocationTemplateInformation},
        invoker::InvokerControlFlow,
    },
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
pub fn eos_objekt_add(name: String) {
    let ret = module_impl::EosObjektType::create_objekt();
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

fn say_dummy() -> Result<InvokerControlFlow, String> {
    println!("dummy invocations, a necessary evil");
    Ok(InvokerControlFlow::Continue)
}

fn say_hello() -> Result<InvokerControlFlow, String> {
    println!("some stuff");
    Ok(InvokerControlFlow::Continue)
}

fn say_loose_node() -> Result<InvokerControlFlow, String> {
    println!("loose invocation");
    Ok(InvokerControlFlow::Continue)
}

#[no_mangle]
pub fn eos_objekt_get_invocations(index: usize) -> InvocationTemplateInformation {
    InvocationTemplateInformation {
        invocation_list: vec![
            InvocationTemplate {
                children: vec![String::from("test")],
                name: String::from("test_link"),
                parents: vec![String::from("test")],
                fn_ptr: say_dummy,
            },
            InvocationTemplate {
                children: vec![],
                name: String::from("test_loose"),
                parents: vec![String::from("test")],
                fn_ptr: say_loose_node,
            },
            InvocationTemplate {
                children: vec![String::from("test_link"), String::from("test_loose")],
                name: String::from("test"),
                parents: vec![String::from("test_link")],
                fn_ptr: say_hello,
            },
        ],
        entrypoint_list: vec![String::from("test")],
    }
}

#[no_mangle]
pub fn eos_objekt_remove(name: &String) {
    unsafe {
        let mut created_objects = CREATED_OBJEKTS.as_mut().unwrap().write();
        created_objects.remove(name);
    }
}

#[no_mangle]
pub fn eos_objekt_remove_all() {
    unsafe {
        let mut created_objects = CREATED_OBJEKTS.as_mut().unwrap().write();
        *created_objects = BTreeMap::new();
    }
}

#[no_mangle]
pub fn eos_objekts_len() -> usize {
    unsafe { CREATED_OBJEKTS.as_mut().unwrap().read().len() }
}
