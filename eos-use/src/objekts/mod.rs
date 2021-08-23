use crate::EosVersion;

pub trait EosModuleObjekt: EosObjekt {
    fn create_objekt() -> Self;
}

pub trait EosObjekt: std::fmt::Debug + mopa::Any + Send {
    //none of the methods in this trait should have mutable access to the objekt
    fn version(&self) -> EosVersion;
}
mopafy!(EosObjekt);
