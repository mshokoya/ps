use serde::{Deserialize, Serialize};
use serde_json::Value;
// use uuid::Uuid;

use crate::libs::{
    db::accounts::types::AccountArg,
    taskqueue::types::{TQTimeout, TaskActionCTX},
};

use super::apollo::confirm::index::apollo_check;

#[derive(Serialize, Debug)]
pub struct Response<T = ()> {
    pub ok: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn ok_none() -> Value {
        serde_json::to_value(Response::<()> {
            ok: true,
            message: None,
            data: None,
        })
        .unwrap()
    }
    pub fn fail_none() -> Value {
        serde_json::to_value(Response::<()> {
            ok: false,
            message: None,
            data: None,
        })
        .unwrap()
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
