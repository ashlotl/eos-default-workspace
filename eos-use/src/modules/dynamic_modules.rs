#![cfg_attr(feature = "use_static_modules", allow(unused))]
use std::{collections::BTreeMap, panic::AssertUnwindSafe, sync::Arc};

use parking_lot::RwLock;

use crate::{
    invocations::invocation::InvocationTemplateInformation,
    modules::{EosModuleConfig, EosModuleFnTypes},
    objekts::EosObjekt,
    utils::errors,
};

#[derive(Debug)]
pub struct EosModuleDynamic {
    pub config: EosModuleConfig,

    pub lib: Arc<libloading::Library>,

    pub module_init: libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnModuleInit>,
    pub objekt_add: libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnObjektAdd>,
    pub objekt_get: libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnObjektGet>,
    pub objekt_get_invocations:
        libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnObjektGetInvocations>,
    pub objekt_remove: libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnObjektRemove>,
    pub objekt_remove_all:
        libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnObjektRemoveAll>,
    pub objekts_len: libloading::Symbol<'static, <Self as EosModuleFnTypes>::FnObjektsLen>,
}

impl EosModuleFnTypes for EosModuleDynamic {
    type FnModuleInit =
        extern "C" fn(module_list_ptr: Arc<RwLock<BTreeMap<String, Arc<EosModuleDynamic>>>>);
    type FnObjektAdd = fn(name: String);
    type FnObjektGet = fn(name: &String) -> (Arc<dyn EosObjekt>, usize);
    type FnObjektGetInvocations = fn(usize) -> InvocationTemplateInformation;
    type FnObjektRemove = fn(name: &String);
    type FnObjektRemoveAll = fn();
    type FnObjektsLen = fn() -> usize;
}

impl EosModuleDynamic {
    pub fn call_module_init(
        &self,
        module_list_ref: Arc<RwLock<BTreeMap<String, Arc<EosModuleDynamic>>>>,
    ) -> Result<(), String> {
        let module_list_ref = AssertUnwindSafe(module_list_ref);
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.module_init)((*module_list_ref).clone())
        }))
    }

    pub fn call_objekt_add(&self, name: String) -> Result<(), String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_add)(name)
        }))
    }

    pub fn call_objekt_get_invocations(
        &self,
        index: usize,
    ) -> Result<InvocationTemplateInformation, String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_get_invocations)(index)
        }))
    }

    pub fn call_objekt_remove(&self, name: &String) -> Result<(), String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_remove)(name)
        }))
    }

    pub fn call_objekt_remove_all(&self) -> Result<(), String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_remove_all)()
        }))
    }

    pub fn call_objekts_len(&self) -> Result<usize, String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekts_len)()
        }))
    }
}
