use core::fmt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
// https://stackoverflow.com/a/65040451/5252283
use uuid::Uuid;

#[derive(Clone, Serialize, Debug)]
pub struct ActionData {
    pub task_group: TaskGroup,
    pub task_type: TaskType,
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum TaskGroup {
    Enqueue,
    Dequeue,
    Apollo,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum TaskType {
    Enqueue,
    Dequeue,
    ApolloConfirm,
}

#[derive(Debug)]
pub enum Channels {
    WaitQueue,
    ProcessQueue,
    TimeoutQueue,
    Apollo,
}

impl Into<&str> for Channels {
    fn into(self) -> &'static str {
        match self {
            Channels::WaitQueue => "waitQueue",
            Channels::ProcessQueue => "processQueue",
            Channels::TimeoutQueue => "timeoutQueue",
            Channels::Apollo => "apollo",
        }
    }
}

#[derive(Clone, Serialize, Debug)]
pub struct TaskEvent {
    pub task_id: Uuid,
    pub message: &'static str,
    pub task_type: TaskType,
    pub metadata: Option<Arc<HashMap<String, Value>>>,
    pub action_data: ActionData,
}

#[derive(Serialize, Debug)]
pub struct Response<T> {
    pub ok: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug)]
pub struct History {
    pub total_page_scrape: u8,
    pub scrape_time: Option<u64>,
    pub list_name: String,
    pub scrape_id: Uuid,
}

#[derive(Debug)]
pub struct Account {
    id: Uuid,
    domain: String, // enum Domain
    trial_time: Option<u64>,
    suspended: bool,
    login_type: String, // enum
    verified: bool,
    email: String,
    password: String,
    proxy: String,
    credits_used: Option<i16>,
    credit_limit: Option<i16>,
    renewal_date: Option<u64>,
    renewal_start_date: Option<u64>,
    renewal_end_date: Option<u64>,
    trial_days_left: Option<u64>,
    last_used: Option<u64>,
    cookies: Option<String>,
    history: History,
}

#[derive(Debug, Deserialize)]
pub struct HistoryArg {
    pub total_page_scrape: Option<u8>,
    pub scrape_time: Option<u64>,
    pub list_name: Option<String>,
    pub scrape_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ConfirmTaskArgs {
    pub account: AccountArg,
    pub timeout: String,
}

#[derive(Debug, Deserialize)]
pub enum TaskArgs {
    ApolloConfirm(AccountArg),
}
