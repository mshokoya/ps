use polodb_core::bson::{to_document, doc};
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::Response as R,
    libs::db::{
            accounts::types::Account,
            entity::Entity,
            index::DB,
        },
};


#[tauri::command]
pub fn delete_accounts(ctx: AppHandle, args: Value) -> R {
  let args = doc! {"$or": to_document(&args).unwrap() };
  match ctx.state::<DB>().find::<Account>(Entity::Account, Some(args)) {
    Ok(docs) => R::ok_data(to_document(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}