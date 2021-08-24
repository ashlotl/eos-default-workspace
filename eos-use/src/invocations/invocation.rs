use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{Condvar, Mutex, RwLock};

use crate::{invocations::invoker::InvokerControlFlow, objekts::EosObjekt};

pub type WaitType = Arc<(Mutex<bool>, Condvar)>;

#[derive(Clone, Debug)]
pub struct Invocation {
    pub children: Vec<WaitType>,
    pub fn_ptr: fn(Arc<RwLock<dyn EosObjekt>>) -> Result<InvokerControlFlow, String>,
    pub objekt_getter: fn(String) -> Result<Arc<RwLock<dyn EosObjekt>>, String>,
    pub objekt_name: String,
    pub parents: Vec<WaitType>,
}

impl Invocation {
    pub fn wait_for_parents(&self) {
        for parent in &self.parents {
            println!("locking");
            let mut guard = parent.0.lock();
            println!("locked");
            if *guard == true {
                println!("skipping");
                *guard = false;
            } else {
                println!("waiting");
                parent.1.wait(&mut guard);
                println!("past");
            }
        }
    }

    pub fn signal_children(&self) {
        for child in &self.children {
            // let mut guard = child.0.lock();
            // *guard = false;
            while !child.1.notify_one() {}
        }
    }
}

#[derive(Clone)]
pub struct InvocationTemplate {
    pub children: Vec<String>,
    pub fn_ptr: fn(Arc<RwLock<dyn EosObjekt>>) -> Result<InvokerControlFlow, String>,
    pub objekt_getter: fn(String) -> Result<Arc<RwLock<dyn EosObjekt>>, String>,
    pub name: String,
    pub objekt_name: String,
    pub parents: Vec<String>,
}

pub struct InvocationTemplateInformation {
    pub invocation_list: Vec<InvocationTemplate>,
    pub entrypoint_list: Vec<String>,
}
