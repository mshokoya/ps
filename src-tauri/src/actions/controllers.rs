use polodb_core::bson::Document;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
// use uuid::Uuid;

use crate::libs::{
    db::accounts::types::AccountArg,
    taskqueue::types::{TQTimeout, TaskActionCTX},
};

use super::apollo::check::index::apollo_check;

#[derive(Serialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub message: Option<String>,
    pub data: Option<Document>,
}

impl Response {
    pub fn ok_none() -> Self {
        Response {
            ok: true,
            message: None,
            data: None,
        }
    }

    pub fn ok_data(data: Document ) -> Self {
        Response{
            ok: true,
            message: None,
            data: Some(data)
        }
    }

    pub fn fail_none() -> Self {
        Response {
            ok: false,
            message: None,
            data: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfirmTaskArgs {
    pub account: AccountArg,
    pub timeout: Option<TQTimeout>,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum TaskType {
    Enqueue,
    Dequeue,
    ApolloCheck,
}

impl TaskType {
    pub async fn exec(
        &self,
        ctx: TaskActionCTX,
        args: Option<Value>,
    ) -> anyhow::Result<Option<Value>> {
        match self {
            TaskType::ApolloCheck => apollo_check(ctx, args).await,
            _ => Ok(None),
        }
    }
}
