use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    actions::controllers::{Response as R, TaskType},
    libs::{
        scraper::Scraper,
        taskqueue::{
            index::TaskQueue,
            types::{TQTimeout, Task, TaskGroup},
        },
    },
    SCRAPER,
};

use super::types::ApolloCheckArgs;

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

pub async fn apollo_check(ctx: AppHandle, task_id: String, args: Value) -> Option<()> {
    let args: ApolloCheckArgs = serde_json::from_value(args).unwrap();

    let page = unsafe { SCRAPER.incog().await.unwrap() };

    page.goto("https://crates.io/search?q=chromium&sort=downloads")
        .await.unwrap()
        .goto("https://www.youtube.com")
        .await.unwrap()
        .goto("https://dash.cloudflare.com/login")
        .await.unwrap()
        .goto("https://profy.dev/project/react-job-simulator/welcome")
        .await.unwrap()
        .goto("https://www.youtube.com/watch?v=Ahwoks_dawU")
        .await.unwrap()
        .goto("https://docs.rs/chromiumoxide/0.5.0/chromiumoxide/handler/struct.Handler.html#method.try_poll_next")
        .await.unwrap()
        .goto("https://www.game.com")
        .await.unwrap()
        .goto("https://www.sitelike.org/similar/downduck.com/")
        .await.unwrap();

    todo!();
}
