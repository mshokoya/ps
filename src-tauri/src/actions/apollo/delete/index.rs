use polodb_core::bson::{to_document, Document};
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
pub fn delete_accounts(ctx: AppHandle, args: Document) -> R {
  match ctx.state::<DB>().find::<Account>(Entity::Account, Some(args)) {
    Ok(docs) => R::ok_data(to_document(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}