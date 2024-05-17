use polodb_core::bson::oid::ObjectId;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ApolloCheckArgs {
    account_id: Uuid,
}
