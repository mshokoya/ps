use super::accounts::types::Account;
use super::domain::types::Domain;
use super::entity::{Entity, EntityTrait};
use super::metadata::types::Metadata;
use super::proxy::types::Proxy;
use super::records::types::{Record, Records};
use anyhow::Result;
use polodb_core::bson::oid::ObjectId;
use polodb_core::bson::to_document;
use polodb_core::bson::{doc, from_document};
use polodb_core::results::DeleteResult;
use polodb_core::Collection;
use polodb_core::{bson::Document, Database};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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
        db.collection::<Records>("records");
        db.collection::<Metadata>("metadata");
        db.collection::<Proxy>("proxy");
    }

    fn get_collection(&self, entity_name: &'static str) -> Collection<Document> {
        self.db.lock().unwrap().collection::<Document>(entity_name)
    }

    pub fn insert_one<T: EntityTrait + Serialize>(&self, entity: Entity, doc: T) -> Result<String> {
        let doc = entity.validate(doc).unwrap();
        let collection = self.get_collection(entity.name());
        let doc = to_document(&doc).unwrap();
        let result = collection.insert_one(doc)?;

        Ok(result.inserted_id.to_string())
    }

    pub fn find<T: DeserializeOwned>(
        &self,
        entity: Entity,
        filter: Option<Document>,
    ) -> Result<Vec<T>> {
        Ok(self
            .get_collection(entity.name())
            .find(filter.unwrap_or(doc! {}))?
            .map(|entity| from_document::<T>(entity.unwrap()).unwrap())
            .collect::<Vec<T>>())
    }

    pub fn find_one<T: DeserializeOwned>(&self, entity: Entity, filter: Document) -> Option<T> {
        match self.get_collection(entity.name()).find_one(filter).unwrap() {
            Some(doc) => Some(from_document(doc).unwrap()),
            None => None,
        }
        // Ok()
    }

    pub fn update_one(&self, entity: Entity, filter: Document, update: Document) -> Result<u64> {
        let collection = self.get_collection(entity.name());
        Ok(collection.update_one(filter, update)?.modified_count)
    }

    pub fn delete_one(&self, entity: Entity, filter: Document) -> Result<DeleteResult> {
        let collection = self.get_collection(entity.name());
        Ok(collection.delete_one(filter)?)
    }
}
