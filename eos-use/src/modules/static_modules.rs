#![cfg_attr(not(feature = "use_static_modules"), allow(unused))]
use std::{collections::BTreeMap, panic::AssertUnwindSafe, sync::Arc};

use parking_lot::RwLock;

use crate::{
    invocations::invocation::InvocationTemplateInformation,
    modules::{EosModuleConfig, EosModuleFnTypes},
    objekts::EosObjekt,
    utils::errors,
};

pub struct EosModuleStatic {
    pub config: EosModuleConfig,

    module_init: <Self as EosModuleFnTypes>::FnModuleInit,
    objekt_add: <Self as EosModuleFnTypes>::FnObjektAdd,
    objekt_get: <Self as EosModuleFnTypes>::FnObjektGet,
    objekt_get_invocations: <Self as EosModuleFnTypes>::FnObjektGetInvocations,
    objekt_remove: <Self as EosModuleFnTypes>::FnObjektRemove,
    objekt_remove_all: <Self as EosModuleFnTypes>::FnObjektRemoveAll,
    objekts_len: <Self as EosModuleFnTypes>::FnObjektsLen,
    objekts_get_keys: <Self as EosModuleFnTypes>::FnObjektsGetKeys,
}

impl EosModuleFnTypes for EosModuleStatic {
    type FnModuleInit = fn(module_list_ptr: Arc<RwLock<BTreeMap<String, Arc<EosModuleStatic>>>>);
    type FnObjektAdd = fn(name: String, deserialize_from: String);
    type FnObjektGet = fn(name: &String) -> Arc<RwLock<dyn EosObjekt>>;
    type FnObjektGetInvocations = fn(String) -> InvocationTemplateInformation;
    type FnObjektRemove = fn(name: &String);
    type FnObjektRemoveAll = fn();
    type FnObjektsLen = fn() -> usize;
    type FnObjektsGetKeys = fn() -> Vec<String>;
}

impl EosModuleStatic {
    pub fn new(
        module_init: <Self as EosModuleFnTypes>::FnModuleInit,
        objekt_add: <Self as EosModuleFnTypes>::FnObjektAdd,
        objekt_get: <Self as EosModuleFnTypes>::FnObjektGet,
        objekt_get_invocations: <Self as EosModuleFnTypes>::FnObjektGetInvocations,
        objekt_remove: <Self as EosModuleFnTypes>::FnObjektRemove,
        objekt_remove_all: <Self as EosModuleFnTypes>::FnObjektRemoveAll,
        objekts_len: <Self as EosModuleFnTypes>::FnObjektsLen,
        objekts_get_keys: <Self as EosModuleFnTypes>::FnObjektsGetKeys,
    ) -> Self {
        Self {
            config: EosModuleConfig {
                name: String::from("default"),
            },
            module_init,
            objekt_add,
            objekt_get,
            objekt_get_invocations,
            objekt_remove,
            objekt_remove_all,
            objekts_len,
            objekts_get_keys,
        }
    }

    pub fn call_module_init(
        &self,
        module_list_ref: Arc<RwLock<BTreeMap<String, Arc<EosModuleStatic>>>>,
    ) -> Result<(), String> {
        let module_list_ref = AssertUnwindSafe(module_list_ref);
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.module_init)((*module_list_ref).clone())
        }))
    }

    pub fn call_objekt_add(&self, name: String, deserialize_from: String) -> Result<(), String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_add)(name, deserialize_from)
        }))
    }

    pub fn call_objekt_get(&self, name: &String) -> Result<Arc<RwLock<dyn EosObjekt>>, String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_get)(name)
        }))
    }

    pub fn call_objekt_get_invocations(
        &self,
        name: String,
    ) -> Result<InvocationTemplateInformation, String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekt_get_invocations)(name)
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

    pub fn call_objekts_get_keys(&self) -> Result<Vec<String>, String> {
        errors::convert_result_error_to_string_send(std::panic::catch_unwind(|| {
            (self.objekts_get_keys)()
        }))
    }
}
