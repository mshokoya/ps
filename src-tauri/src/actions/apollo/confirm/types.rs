use polodb_core::bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApolloCheckArgs {
    account_id: ObjectId,
}

// #[derive(Deserialize, Debug)]
// pub struct ApolloCheckArgs {
//     account_id: String,
// }
