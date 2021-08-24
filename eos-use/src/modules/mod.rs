mod dynamic_modules;
mod static_modules;

#[cfg(not(feature = "use_static_modules"))]
pub type EosModuleHandle = dynamic_modules::EosModuleDynamic;

#[cfg(feature = "use_static_modules")]
pub type EosModuleHandle = static_modules::EosModuleStatic;

#[derive(Debug)]
pub struct EosModuleConfig {
    pub name: String,
}

pub trait EosModuleFnTypes {
    type FnModuleInit;
    type FnObjektAdd;
    type FnObjektGet;
    type FnObjektGetInvocations;
    type FnObjektRemove;
    type FnObjektRemoveAll;
    type FnObjektsGetKeys;
    type FnObjektsLen;
}
