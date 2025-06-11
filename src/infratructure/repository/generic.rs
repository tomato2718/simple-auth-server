use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GenericTableManager<T> {
    table: Arc<Mutex<HashMap<String, T>>>,
}

impl<T> GenericTableManager<T> {
    pub fn new() -> Self {
        GenericTableManager {
            table: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_table(&self) -> Arc<Mutex<HashMap<String, T>>> {
        self.table.clone()
    }
}
