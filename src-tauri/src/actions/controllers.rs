use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::AppHandle;
use uuid::Uuid;

use crate::libs::taskqueue::types::TQTimeout;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountArg {
    pub id: Option<Uuid>,
    pub domain: Option<String>, // enum Domain
    pub trial_time: Option<u64>,
    pub suspended: Option<bool>,
    pub login_type: Option<String>, // enum
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
    pub credits_used: Option<u16>,
    pub credit_limit: Option<u16>,
    pub renewal_date: Option<u64>,
    pub renewal_start_date: Option<u64>,
    pub renewal_end_date: Option<u64>,
    pub trial_days_left: Option<u64>,
    pub last_used: Option<u64>,
    pub cookies: Option<String>,
    pub history: Option<HistoryArg>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryArg {
    pub total_page_scrape: Option<u8>,
    pub scrape_time: Option<u64>,
    pub list_name: Option<String>,
    pub scrape_id: Option<Uuid>,
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
        ctx: &AppHandle,
        task_id: &Uuid,
        args: Option<Value>,
    ) -> anyhow::Result<Option<Value>> {
        match self {
            TaskType::ApolloCheck => apollo_check(ctx, task_id, args).await,
            _ => Ok(None),
        }
    }
}
