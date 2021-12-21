use std::{
    sync::{
        Arc, Mutex,
    }
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Store {
    values: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            values: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn add(&self, key: String, value: Vec<u8>) {
        let mut values = self.values.lock().unwrap();
        values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let values = self.values.lock().unwrap();
        values.get(key).map(|f| f.to_vec())
    }
}