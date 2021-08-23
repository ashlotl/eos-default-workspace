use std::sync::Arc;

use parking_lot::{Condvar, Mutex};

use super::invoker::InvokerControlFlow;

pub type WaitType = Arc<(Mutex<bool>, Condvar)>;

#[derive(Clone, Debug)]
pub struct Invocation {
    pub children: Vec<WaitType>,
    pub fn_ptr: fn() -> Result<InvokerControlFlow, String>,
    pub parents: Vec<WaitType>,
}

impl Invocation {
    pub fn wait_for_parents(&self) {
        for parent in &self.parents {
            let mut guard = parent.0.lock();
            if *guard == true {
                *guard = false;
            } else {
                parent.1.wait(&mut guard);
            }
        }
    }

    pub fn signal_children(&self) {
        for child in &self.children {
            // let mut guard = child.0.lock();
            // *guard = false;
            while child.1.notify_all() == 0 {}
        }
    }
}

#[derive(Clone)]
pub struct InvocationTemplate {
    pub children: Vec<String>,
    pub fn_ptr: fn() -> Result<InvokerControlFlow, String>,
    pub name: String,
    pub parents: Vec<String>,
}

pub struct InvocationTemplateInformation {
    pub invocation_list: Vec<InvocationTemplate>,
    pub entrypoint_list: Vec<String>,
}
