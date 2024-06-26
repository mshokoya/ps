use polodb_core::bson::{serde_helpers::serialize_object_id_as_hex_string, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize)]
pub struct Domain {
    pub _id: String,
    pub domain: String,
    pub verified: bool,
    pub mx_records: bool,
    pub txt_records: bool,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainArg {
    pub _id: Option<String>,
    pub domain: Option<String>,
    verified: Option<bool>,
    mx_records: Option<bool>,
    txt_records: Option<bool>,
    message: Option<String>,
}