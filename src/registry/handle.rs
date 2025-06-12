use std::collections::HashMap;
use crate::validation::ValidationError;

#[derive(Debug, Default)]
pub struct HandleRegistry {
    handles: HashMap<String, String>, // @name.fd → address
}

impl HandleRegistry {
    pub fn new() -> Self {
        HandleRegistry {
            handles: HashMap::new(),
        }
    }

    pub fn register(&mut self, handle: &str, owner: &str) -> Result<(), ValidationError> {
        if let Some(current_owner) = self.handles.get(handle) {
            if current_owner != owner {
                return Err(ValidationError::UnauthorizedNode);
            }
        }
        self.handles.insert(handle.to_string(), owner.to_string());
        Ok(())
    }

    pub fn delete(&mut self, handle: &str, owner: &str) -> Result<(), ValidationError> {
        if let Some(current_owner) = self.handles.get(handle) {
            if current_owner == owner {
                self.handles.remove(handle);
                return Ok(());
            } else {
                return Err(ValidationError::UnauthorizedNode);
            }
        }
        Err(ValidationError::InvalidHandleFormat)
    }

    pub fn resolve(&self, handle: &str) -> Option<&String> {
        self.handles.get(handle)
    }

    pub fn print_all(&self) {
        for (h, a) in &self.handles {
            println!("{} → {}", h, a);
        }
    }
}
