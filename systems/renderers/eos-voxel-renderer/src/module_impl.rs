use std::sync::Arc;

use eos_use::{
    invocations::{
        invocation::{InvocationTemplate, InvocationTemplateInformation},
        invoker::InvokerControlFlow,
    },
    objekts::{EosModuleObjekt, EosObjekt},
    EosVersion,
};

use parking_lot::{Mutex, RwLock};

use serde::Deserialize;

pub type EosObjektType = VoxelRenderer;

#[derive(Debug, Deserialize)]
pub struct VoxelRenderer {
    pub poisoned: bool,
    pub tick: u32,
    pub version: EosVersion,
}

impl EosModuleObjekt for VoxelRenderer {
    //all associated fns
    fn create_objekt(deserialize_from: String) -> Result<Self, ron::Error> {
        unsafe { ron::from_str(&*(deserialize_from.as_str() as *const str)) }
    }
}

impl EosObjekt for VoxelRenderer {
    fn get_invocations(
        &self,
        objekt_name: String,
        objekt_getter: fn(String) -> Result<Arc<RwLock<dyn EosObjekt>>, String>,
    ) -> InvocationTemplateInformation {
        InvocationTemplateInformation {
            invocation_list: vec![
                InvocationTemplate {
                    children: vec![format!("{}test", objekt_name)],
                    name: format!("{}test_link", objekt_name),
                    parents: vec![format!("{}test", objekt_name)],
                    objekt_getter: objekt_getter,
                    objekt_name: objekt_name.clone(),
                    fn_ptr: say_dummy,
                },
                InvocationTemplate {
                    children: vec![],
                    name: format!("{}test_loose", objekt_name),
                    parents: vec![format!("{}test", objekt_name)],
                    objekt_getter: objekt_getter,
                    objekt_name: objekt_name.clone(),
                    fn_ptr: say_loose_node,
                },
                InvocationTemplate {
                    children: vec![
                        format!("{}test_link", objekt_name),
                        format!("{}test_loose", objekt_name),
                    ],
                    name: format!("{}test", objekt_name),
                    parents: vec![format!("{}test_link", objekt_name)],
                    objekt_getter: objekt_getter,
                    objekt_name: objekt_name.clone(),
                    fn_ptr: say_hello,
                },
            ],
            entrypoint_list: vec![format!("{}test", objekt_name)],
        }
    }
    fn poisoned(&self) -> bool {
        self.poisoned
    }

    fn set_poisoned(&mut self) {
        self.poisoned = true;
    }

    fn version(&self) -> EosVersion {
        self.version.clone()
    }
}

fn say_dummy(_objekt: Arc<RwLock<dyn EosObjekt>>) -> Result<InvokerControlFlow, String> {
    // println!("dummy invocations, a necessary evil");
    Ok(InvokerControlFlow::Continue)
}

fn say_hello(_objekt: Arc<RwLock<dyn EosObjekt>>) -> Result<InvokerControlFlow, String> {
    // println!("some stuff");
    Ok(InvokerControlFlow::Continue)
}

fn say_loose_node(objekt: Arc<RwLock<dyn EosObjekt>>) -> Result<InvokerControlFlow, String> {
    let mut guard = objekt.write();
    let contents: &mut VoxelRenderer = guard.downcast_mut().unwrap();
    // println!("tick: {}", contents.tick);
    if contents.tick == 10 {
        println!("error time");
        panic!("big dummy error");
    }
    contents.tick += 1;
    // println!("loose invocation");
    Ok(InvokerControlFlow::Continue)
}
