use std::path::Path;
pub use sled::Db;
pub use sled::Error as SledError;
pub use sled::Tree;
pub use sled::IVec;

pub struct KVStore {
    tree: Tree,
}

impl KVStore {
    pub fn new(tree: Tree) -> Self {
        Self { tree }
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<IVec>, SledError> {
        self.tree.get(key)
    }

    pub fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, SledError> {
        self.tree.insert(key, value)
    }

    pub fn remove(&self, key: &[u8]) -> Result<Option<IVec>, SledError> {
        self.tree.remove(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<(IVec, IVec), SledError>> + '_ {
        self.tree.iter()
    }
}
