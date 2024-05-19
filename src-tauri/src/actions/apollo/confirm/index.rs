use std::sync::Arc;

use anyhow::Error;
use polodb_core::bson::{doc, Uuid};
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::actions::apollo::lib::index::apollo_login_credits_info;
use crate::libs::taskqueue::types::Channels;
use crate::{
    actions::controllers::{Response as R, TaskType},
    libs::{
        db::{
            accounts::types::{Account, AccountArg},
            entity::Entity,
            index::DB,
            records::types::{Record, RecordArg, Records, RecordsArg},
        },
        taskqueue::{
            index::TaskQueue,
            types::{TQTimeout, Task, TaskActionCTX, TaskGroup},
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

    let fmt_args = match args.get("account_id") {
        Some(_) => Some(args.to_owned()),
        None => return R::<()>::fail_none(),
    };

    ctx.state::<TaskQueue>().w_enqueue(Task {
        task_id: Uuid::new(),
        task_type: TaskType::ApolloCheck,
        task_group: TaskGroup::Apollo,
        message: "Getting credits",
        metadata,
        timeout,
        args: fmt_args,
    });

    R::<()>::ok_none()
}

pub async fn apollo_check(
    mut ctx: TaskActionCTX,
    args: Option<Value>,
) -> Result<Option<Value>, Error> {
    let args: ApolloCheckArgs = serde_json::from_value(args.unwrap())?;
    let account = ctx
        .handle
        .state::<DB>()
        .find_one::<Account>(Entity::Account, doc! {"_id": args.account_id})
        .unwrap();

    ctx.page = Some(unsafe { SCRAPER.incog().await? });

    // apollo_login_then_visit(
    //     ctx,
    //     &account,
    //     "https://app.apollo.io/#/settings/credits/current",
    // )
    // .await;

    ctx.handle
        .emit(
            Channels::Apollo.into(),
            doc! {"taskID": ctx.task_id, "message": "navigated to the credits page"},
        )
        .unwrap();

    // apollo_login_credits_info(ctx, args).await;

    ctx.handle
        .emit(
            Channels::Apollo.into(),
            doc! {"taskID": ctx.task_id, "message": format!("successfully obtained {} credits info", account.email)},
        )
        .unwrap();

    todo!()
    // Ok(Some(to_value(doc!({"":""}))))
}

// page.goto("https://crates.io/search?q=chromium&sort=downloads")
//     .await?
//     .goto("https://www.youtube.com")
//     .await?
//     .goto("https://dash.cloudflare.com/login")
//     .await?
//     .goto("https://profy.dev/project/react-job-simulator/welcome")
//     .await?
//     .goto("https://www.youtube.com/watch?v=Ahwoks_dawU")
//     .await?
//     .goto("https://docs.rs/chromiumoxide/0.5.0/chromiumoxide/handler/struct.Handler.html#method.try_poll_next")
//     .await?
//     .goto("https://www.game.com")
//     .await?
//     .goto("https://www.sitelike.org/similar/downduck.com/")
//     .await?;

// Ok(Some(
//     serde_json::to_value(AccountArg {
//         // _id: Some(Uuid::new_v4()),
//         _id: Some(ObjectId::parse_str("66477168cf323e7729dc753f").unwrap()),
//         domain: Some("test value domain".to_string()), // enum Domain
//         trial_time: Some(342),
//         suspended: Some(false),
//         login_type: Some("test value login_type".to_string()), // enum
//         verified: Some(true),
//         email: Some("test value email".to_string()),
//         password: Some("test value password".to_string()),
//         proxy: Some("test value proxy".to_string()),
//         credits_used: Some(342),
//         credit_limit: Some(342),
//         renewal_date: Some(342),
//         renewal_start_date: Some(342),
//         renewal_end_date: Some(342),
//         trial_days_left: Some(342),
//         last_used: Some(342),
//         cookies: None,
//         history: None,
//     })
//     .unwrap(),
// ))
