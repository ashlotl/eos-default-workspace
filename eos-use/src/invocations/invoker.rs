use std::{collections::BTreeMap, panic::AssertUnwindSafe, sync::Arc, thread};

use parking_lot::{Condvar, Mutex};

use crate::{
    invocations::invocation::{Invocation, InvocationTemplate},
    utils::errors,
};

#[derive(Clone, Debug)]
pub struct BuiltInvoker {
    invocations: Vec<(String, Arc<Mutex<Invocation>>)>,
}

impl BuiltInvoker {
    fn run_impl(&self) {
        let mut handles = vec![];

        println!("{:?}", self.invocations);

        for invocation in &self.invocations {
            let invocation = invocation.clone();
            handles.push(thread::spawn(move || {
                #[allow(unused_assignments)]
                let mut control_flow = InvokerControlFlow::Continue(false);
                let name = invocation.0;
                let invocation = invocation.1.lock();
                let objekt = match (invocation.objekt_getter)(invocation.objekt_name.clone()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("A thread was stopped: {}", e);
                        return InvokerControlFlow::Stop;
                    }
                };
                let fn_ptr = invocation.fn_ptr;
                loop {
                    println!("invocation: {}", name);
                    control_flow = invocation.wait_for_parents();
                    println!("{} has parent go-ahead", name);
                    if let InvokerControlFlow::Continue(_) = control_flow {
                    } else {
                        println!("stop");
                        invocation.signal_children(control_flow.clone());
                        break;
                    }
                    let guard = objekt.read();
                    let poisoned = (*guard).poisoned();
                    std::mem::drop(guard);
                    if !poisoned {
                        let objekt_safe = AssertUnwindSafe(objekt.clone());
                        let res = std::panic::catch_unwind(move || fn_ptr((*objekt_safe).clone()));

                        match res {
                            Ok(Ok(v)) => {
                                if invocation.children.len() == 0 {
                                    if invocation.parents.len() == 0 {
                                        break;
                                    }
                                    continue;
                                }
                                println!("{} is signaling children", name);
                                invocation.signal_children(v);
                            }
                            Ok(Err(v)) => {
                                let mut guard = objekt.write();
                                guard.set_poisoned();
                                println!(
                                    "Invocation `{}` reported an error, stopping executions: `{}`",
                                    name, v
                                );
                                if invocation.children.len() == 0 {
                                    if invocation.parents.len() == 0 {
                                        break;
                                    }
                                    continue;
                                }
                                invocation.signal_children(InvokerControlFlow::Continue(false));
                            }
                            Err(e) => {
                                let mut guard = objekt.write();
                                guard.set_poisoned();
                                println!(
                                    "Error in invocation `{}`, stopping executions: `{}`",
                                    name,
                                    errors::convert_error_to_string(e)
                                );
                                if invocation.children.len() == 0 {
                                    if invocation.parents.len() == 0 {
                                        break;
                                    }
                                    continue;
                                }
                                invocation.signal_children(InvokerControlFlow::Continue(false));
                            }
                        };
                    }
                }
                return control_flow;
            }));
        }

        let mut control_flow = InvokerControlFlow::Continue(false);

        for handle in handles {
            let invocation_flow = handle.join().unwrap();
            match control_flow {
                InvokerControlFlow::Continue(_) => {
                    control_flow = invocation_flow;
                }
                InvokerControlFlow::Restart(_) => {
                    if let InvokerControlFlow::Continue(_) = invocation_flow {
                    } else {
                        control_flow = invocation_flow;
                    }
                }
                InvokerControlFlow::Stop => {
                    if let InvokerControlFlow::Stop = invocation_flow {
                        control_flow = invocation_flow;
                    }
                }
            }
        }

        match control_flow {
            InvokerControlFlow::Continue(_) => {
                //Threads stopped through hanging nodes (nodes without children)
                return;
            }
            InvokerControlFlow::Restart(v) => {
                v.run_impl();
            }
            InvokerControlFlow::Stop => {
                //Threads stopped through invocation logic
                return;
            }
        };
    }

    pub async fn run(self) {
        self.run_impl();
    }
}

pub struct Invoker {
    entrypoints: Vec<String>,
    invocations: BTreeMap<String, InvocationTemplate>,
}

impl Invoker {
    pub fn new() -> Self {
        Self {
            entrypoints: vec![],
            invocations: BTreeMap::new(),
        }
    }

    pub fn check_for_errors_and_build(self) -> Result<BuiltInvoker, String> {
        //check for doubled entrypoints and entrypoints without invocations:
        let invocations = self.invocations;
        let entrypoints = self.entrypoints;
        for i in 0..entrypoints.len() {
            if !invocations.contains_key(&entrypoints[i]) {
                return Err(String::from(format!(
                    "Entrypoint `{}` is not found in invocation map",
                    entrypoints[i]
                )));
            }
            for j in 0..entrypoints.len() {
                if i != j && entrypoints[i] == entrypoints[j] {
                    return Err(String::from(format!(
                        "Duplicate entrypoints: `{}` at position {} and {}",
                        entrypoints[i], i, j
                    )));
                }
            }
        }

        //check for loose invocations, references to non-existant invocations, doubled invocations, and parents that do not reciprocate children/ children that do not reciprocate parents
        let mut errors = vec![];
        for entry_i in invocations.iter() {
            let invocation = entry_i.1;
            if invocation.parents.len() == 0 && !entrypoints.contains(entry_i.0) {
                errors.push(format!("Invocation `{}` is unreachable", entry_i.0));
            }
            for parent in &invocation.parents {
                let mut ref_count = 0;
                for entry_j in invocations.iter() {
                    if entry_j.0 != &invocation.name && entry_j.0 == parent {
                        if !entry_j.1.children.contains(&invocation.name) {
                            errors.push(format!("The invocation `{}` lists a parent `{}`, but the listed children of that parent don't include `{}`. It may have been overriden by a node with a duplicate name", entry_i.0, parent, entry_i.0));
                        }
                        ref_count += 1;
                    }
                }
                if ref_count == 0 {
                    errors.push(format!(
                        "Invocation `{}` can't find parent with name `{}`",
                        entry_i.0, parent,
                    ));
                }
                if ref_count > 1 {
                    errors.push(format!(
                        "The invocation name `{}` was found more than once",
                        parent
                    ));
                }
            }

            for child in &invocation.children {
                let mut child_matches = 0;
                for entry_j in invocations.iter() {
                    if entry_j.0 != &invocation.name && entry_j.0 == child {
                        println!("{} matched child with {}", invocation.name, entry_j.0);
                        if !entry_j.1.parents.contains(&invocation.name) {
                            println!("adding error");
                            errors.push(format!("The invocation `{}` lists a child `{}`, but the listed parents of that child don't include `{}`. It may have been overriden by a node with a duplicate name", entry_i.0, child, entry_i.0));
                        }
                        child_matches += 1;
                    }
                }
                if child_matches == 0 {
                    errors.push(format!(
                        "Invocation \"{}\" can't find child with name `{}`",
                        entry_i.0, child
                    ));
                }
                if child_matches > 1 {
                    errors.push(format!(
                        "The invocation name `{}` was found more than once",
                        child
                    ));
                }
            }
        }

        if errors.len() > 0 {
            let mut ret_string = String::from("");
            for error in errors {
                ret_string.push_str(error.as_str());
                ret_string.push('\n');
            }
            return Err(ret_string);
        }

        //if we've gotten this far, the invoker should be safe to use
        let mut built = BuiltInvoker {
            invocations: vec![],
        };

        let mut building_map: BTreeMap<String, Invocation> = BTreeMap::new();

        for template_name in invocations.keys() {
            let template = invocations.get(template_name).unwrap();

            let mut building = if let Some(val) = building_map.get(template_name) {
                val.clone()
            } else {
                println!("Adding to building map template: {}", template_name);
                let put = Invocation {
                    children: vec![],
                    fn_ptr: template.fn_ptr,
                    objekt_getter: template.objekt_getter,
                    objekt_name: template.objekt_name.clone(),
                    parents: vec![],
                };
                building_map.insert(template_name.clone(), put.clone());
                put
            };

            for parent in &template.parents {
                let template_building = invocations.get(parent).unwrap();
                let mut parent_building = if let Some(val) = building_map.get(parent) {
                    val.clone()
                } else {
                    println!("Adding to building map parent: {}", parent);
                    let put = Invocation {
                        children: vec![],
                        fn_ptr: template_building.fn_ptr,
                        objekt_getter: template_building.objekt_getter,
                        objekt_name: template_building.objekt_name.clone(),
                        parents: vec![],
                    };
                    building_map.insert(parent.clone(), put.clone());
                    put
                };
                let wait = Arc::new((
                    Mutex::new(InvokerControlFlow::Continue(false)),
                    Condvar::new(),
                ));
                println!("pushing a wait");
                parent_building.children.push(wait.clone());
                building_map.insert(parent.clone(), parent_building);
                building.parents.push(wait.clone());
            }

            building_map.insert(template_name.clone(), building.clone());
        }

        for entrypoint in &entrypoints {
            println!("entrypoint: {}", entrypoint);
            for parent in &building_map.get(entrypoint).unwrap().parents {
                println!("notifying parent");
                *parent.0.lock() = InvokerControlFlow::Continue(true);
            }
        }

        for entry in building_map {
            built
                .invocations
                .push((entry.0, Arc::new(Mutex::new(entry.1))));
        }

        Ok(built)
    }

    pub fn push_entrypoints(&mut self, mut entrypoints: Vec<String>) {
        self.entrypoints.append(&mut entrypoints);
    }

    pub fn push_invocations(&mut self, invocations: Vec<InvocationTemplate>) {
        let map = &mut self.invocations;
        for e in invocations {
            map.insert(e.name.clone(), e);
        }
    }
}

#[derive(Clone, Debug)]
pub enum InvokerControlFlow {
    Continue(bool),
    Stop,
    Restart(Box<BuiltInvoker>),
}
