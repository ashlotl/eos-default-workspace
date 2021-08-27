use std::sync::Arc;

use parking_lot::{Condvar, Mutex, RwLock};

use crate::{invocations::invoker::InvokerControlFlow, objekts::EosObjekt};

pub type WaitType = Arc<(Mutex<InvokerControlFlow>, Condvar)>;

#[derive(Clone, Debug)]
pub struct Invocation {
    pub children: Vec<WaitType>,
    pub fn_ptr: fn(Arc<RwLock<dyn EosObjekt>>) -> Result<InvokerControlFlow, String>,
    pub objekt_getter: fn(String) -> Result<Arc<RwLock<dyn EosObjekt>>, String>,
    pub objekt_name: String,
    pub parents: Vec<WaitType>,
}

impl Invocation {
    pub fn wait_for_parents(&self) -> InvokerControlFlow {
        let mut control_flow = InvokerControlFlow::Continue(false);
        for parent in &self.parents {
            let mut guard = parent.0.lock();
            control_flow = (*guard).clone();
            if let InvokerControlFlow::Continue(true) = control_flow {
                *guard = InvokerControlFlow::Continue(false);
            } else {
                parent.1.wait(&mut guard);
            }
        }
        control_flow
    }

    pub fn signal_children(&self, control_flow: InvokerControlFlow) {
        let mut done_count = 0;
        let mut done_vec = vec![false; self.children.len()];
        while done_count < self.children.len() {
            for i in 0..self.children.len() {
                if done_vec[i] == false {
                    let child = &self.children[i];
                    let mut guard = child.0.lock();
                    *guard = control_flow.clone();
                    std::mem::drop(guard);
                    if child.1.notify_one() {
                        done_vec[i] = true;
                        done_count += 1;
                    }
                }
            }
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
