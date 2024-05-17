use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use polodb_core::bson::to_document;
use serde_json::Value;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    actions::controllers::{Response as R, TaskType},
    libs::{
        db::{accounts::types::AccountArg, entity::Entity, index::DB},
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
    let metadata = match args.get("account_id") {
        Some(val) => Some(val.to_owned()),
        None => None,
    };

    println!("1");

    let fmt_args = match args.get("account_id") {
        Some(_) => Some(args.to_owned()),
        None => return R::<()>::fail_none(),
    };

    println!("2");

    ctx.state::<DB>().insert_one::<AccountArg>(
        Entity::Account,
        AccountArg {
            _id: None,
            domain: Some("test value domain".to_string()), // enum Domain
            trial_time: Some(342),
            suspended: Some(false),
            login_type: Some("test value login_type".to_string()), // enum
            verified: Some(true),
            email: Some("test value email".to_string()),
            password: Some("test value password".to_string()),
            proxy: Some("test value proxy".to_string()),
            credits_used: Some(342),
            credit_limit: Some(342),
            renewal_date: Some(342),
            renewal_start_date: Some(342),
            renewal_end_date: Some(342),
            trial_days_left: Some(342),
            last_used: Some(342),
            cookies: None,
            history: None,
        },
    );

    // ctx.state::<TaskQueue>().w_enqueue(Task {
    //     task_id: Uuid::new_v4(),
    //     task_type: TaskType::ApolloCheck,
    //     task_group: TaskGroup::Apollo,
    //     message: "Getting credits",
    //     metadata,
    //     timeout,
    //     args: fmt_args,
    // });

    R::<()>::ok_none()
}

pub async fn apollo_check(
    ctx: &AppHandle,
    task_id: &Uuid,
    args: Option<Value>,
) -> Result<Option<Value>, Error> {
    let args: ApolloCheckArgs = serde_json::from_value(args.unwrap())?;
    let page = unsafe { SCRAPER.incog().await? };

    page.goto("https://crates.io/search?q=chromium&sort=downloads")
        .await?
        .goto("https://www.youtube.com")
        .await?
        .goto("https://dash.cloudflare.com/login")
        .await?
        .goto("https://profy.dev/project/react-job-simulator/welcome")
        .await?
        .goto("https://www.youtube.com/watch?v=Ahwoks_dawU")
        .await?
        .goto("https://docs.rs/chromiumoxide/0.5.0/chromiumoxide/handler/struct.Handler.html#method.try_poll_next")
        .await?
        .goto("https://www.game.com")
        .await?
        .goto("https://www.sitelike.org/similar/downduck.com/")
        .await?;

    Ok(Some(
        serde_json::to_value(AccountArg {
            _id: Some(Uuid::new_v4()),
            domain: Some("test value domain".to_string()), // enum Domain
            trial_time: Some(342),
            suspended: Some(false),
            login_type: Some("test value login_type".to_string()), // enum
            verified: Some(true),
            email: Some("test value email".to_string()),
            password: Some("test value password".to_string()),
            proxy: Some("test value proxy".to_string()),
            credits_used: Some(342),
            credit_limit: Some(342),
            renewal_date: Some(342),
            renewal_start_date: Some(342),
            renewal_end_date: Some(342),
            trial_days_left: Some(342),
            last_used: Some(342),
            cookies: None,
            history: None,
        })
        .unwrap(),
    ))
}
