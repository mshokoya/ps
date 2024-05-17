use super::accounts::types::{Account, AccountArg};
use super::domain::types::Domain;
use super::entity::{Entity, EntityTrait};
use anyhow::Result;
use polodb_core::bson::{to_bson, to_document, Bson};
use polodb_core::{bson::Document, Database};
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::io::{Error, ErrorKind};
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
        let db = self.db.lock().unwrap();
        db.collection::<Account>("account");
        db.collection::<Domain>("domain");
        db.collection("records");
        db.collection("metadata");
        db.collection("proxy");
    }

    pub fn insert_one<T: EntityTrait + Serialize>(&self, entity: Entity, doc: T) -> Result<String> {
        let doc = entity.validate(doc).unwrap();

        let collection = self
            .db
            .lock()
            .unwrap()
            .collection::<Document>(entity.name());

        let doc = to_document(&doc).unwrap();
        let result = collection.insert_one(doc)?;
        Ok(result.inserted_id.to_string())
    }

    pub fn find(&self, entity: Entity, filter: Option<Document>) -> Result<Vec<Document>> {
        let collection = self
            .db
            .lock()
            .unwrap()
            .collection::<Document>(entity.name());

        let mut entities = Vec::new();
        let result = collection
            .find(filter.unwrap())?
            .for_each(|entity| entities.push(entity.unwrap()));
        Ok(entities)
    }

    pub fn update_one(&self, entity: Entity, filter: Document, update: Document) -> Result<u64> {
        let collection = self
            .db
            .lock()
            .unwrap()
            .collection::<Document>(entity.name());

        Ok(collection.update_one(filter, update)?.modified_count)
    }
}
