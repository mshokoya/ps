use polodb_core::Database;
use std::sync::{Arc, Mutex};

pub struct DB {
    pub db: Arc<Mutex<Database>>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(Database::open_file("test-polo.db").unwrap())),
        }
    }

    pub fn init(&self) {
        // let db = self.db.lock().unwrap();
        // db.create_collection("account").unwrap();
        // db.create_collection("domain").unwrap();
        // db.create_collection("records").unwrap();
        // db.create_collection("metadata").unwrap();
        // db.create_collection("proxy").unwrap();
    }
}
