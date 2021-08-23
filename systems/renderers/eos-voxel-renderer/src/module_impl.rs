use eos_use::{
    objekts::{EosModuleObjekt, EosObjekt},
    EosVersion,
};
use serde::Deserialize;

const CURRENT_VERSION: EosVersion = EosVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

pub type EosObjektType = VoxelRenderer;

#[derive(Debug, Deserialize)]
pub struct VoxelRenderer {
    pub version: EosVersion,
}

impl EosModuleObjekt for VoxelRenderer {
    //all associated fns
    fn create_objekt() -> Self {
        Self {
            version: CURRENT_VERSION,
        }
    }
}

impl EosObjekt for VoxelRenderer {
    fn version(&self) -> EosVersion {
        self.version.clone()
    }
}
