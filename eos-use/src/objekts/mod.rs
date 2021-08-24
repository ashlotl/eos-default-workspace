use std::sync::Arc;

use parking_lot::RwLock;

use crate::{invocations::invocation::InvocationTemplateInformation, EosVersion};

pub trait EosModuleObjekt: EosObjekt {
    fn create_objekt(deserialize_from: String) -> Result<Self, ron::Error>
    where
        Self: Sized;
}

pub trait EosObjekt: std::fmt::Debug + mopa::Any + Send {
    //none of the methods in this trait should have mutable access to the objekt (save for set_poisoned)
    fn get_invocations(
        &self,
        objekt_name: String,
        objekt_getter: fn(String) -> Result<Arc<RwLock<dyn EosObjekt>>, String>,
    ) -> InvocationTemplateInformation;
    fn poisoned(&self) -> bool; //return if an error occurred for an invocation referencing this objekt
    fn set_poisoned(&mut self);
    fn version(&self) -> EosVersion;
}
mopafy!(EosObjekt);
