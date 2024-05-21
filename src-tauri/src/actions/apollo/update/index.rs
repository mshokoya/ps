use polodb_core::bson::{to_document, Document};
use tauri::{AppHandle, Manager};

use crate::{
    actions::controllers::Response as R,
    libs::db::{
            entity::Entity,
            index::DB,
        },
};


#[tauri::command]
pub fn update_account(ctx: AppHandle, filter: Document, update: Document) -> R {
  match ctx.state::<DB>().update_one(Entity::Account, filter, update) {
    Ok(docs) => R::ok_data(to_document(&docs).unwrap()),
    Err(_) => R::ok_none()
  }
}