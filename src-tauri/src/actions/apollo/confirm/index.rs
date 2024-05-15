use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    actions::controllers::{Response as R, TaskType},
    libs::taskqueue::{
        index::TaskQueue,
        types::{TQTimeout, Task, TaskGroup},
    },
};

#[tauri::command]
pub fn check_task(ctx: AppHandle, args: Value) -> Value {
    let to = args.get("timeout").unwrap().to_owned();
    let timeout: Option<TQTimeout> = serde_json::from_value(to).unwrap_or(None);
    let fmt_args = match args.get("account_id") {
        Some(_) => Some(Arc::new(args.to_owned())),
        None => return R::<()>::fail_none(),
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Uuid::new_v4(),
        task_type: TaskType::ApolloCheck,
        task_group: TaskGroup::Apollo,
        message: "Getting credits",
        metadata: match args.get("account_id") {
            Some(val) => {
                let mut md = HashMap::new();
                md.insert("account_id".to_string(), val.clone());
                Some(Arc::new(md))
            }
            None => None,
        },
        timeout,
        args: fmt_args,
    });

    R::<()>::ok_none()
}

pub fn apollo_check(ctx: AppHandle, task_id: String, args: Value) {}
