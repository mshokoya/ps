use std::sync::Arc;

use serde::Deserialize;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::{
    defaults::{Account, ConfirmTaskArgs, Response, TaskArgs, TaskGroup, TaskType},
    taskqueue::{Task, TaskQueue},
};

// #[derive(Debug, Deserialize)]
// pub struct ConfirmTaskArgs {
//     pub account_id: String,
//     pub timeout: Option<String>,
// }

#[tauri::command]
pub fn confirm_task(state: State<TaskQueue>, args: ConfirmTaskArgs) -> Response<String> {
    state.w_enqueue(Task {
        task_id: Uuid::new_v4(),
        task_group: TaskGroup::Apollo,
        task_type: TaskType::ApolloConfirm,
        message: "confirming account",
        timeout: None,
        metadata: None,
        args: Some(Arc::new(TaskArgs::ApolloConfirm(args.account))),
    });

    Response {
        ok: true,
        message: None,
        data: None,
    }
}

pub fn confirm(handle: AppHandle, task_id: Uuid, account: Account) {}
