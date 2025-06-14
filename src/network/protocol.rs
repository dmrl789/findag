use std::error::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt;

pub struct Protocol {
    handlers: Arc<Mutex<HashMap<String, Box<dyn Fn(&[u8]) -> Result<(), Box<dyn Error>> + Send + Sync>>>>,
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Protocol")
            .field("handlers", &"<opaque>")
            .finish()
    }
}

impl Clone for Protocol {
    fn clone(&self) -> Self {
        Self {
            handlers: Arc::clone(&self.handlers),
        }
    }
}

impl Protocol {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_handler<F>(&self, protocol: &str, handler: F)
    where
        F: Fn(&[u8]) -> Result<(), Box<dyn Error>> + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(protocol.to_string(), Box::new(handler));
    }

    pub fn handle_message(&self, protocol: &str, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let handlers = self.handlers.lock().unwrap();
        if let Some(handler) = handlers.get(protocol) {
            handler(message)
        } else {
            Err("Protocol handler not found".into())
        }
    }
} 