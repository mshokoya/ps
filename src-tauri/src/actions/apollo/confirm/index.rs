use std::{collections::HashMap, sync::Arc};

use anyhow::Error;
use polodb_core::bson::doc;
use polodb_core::bson::oid::ObjectId;
use polodb_core::bson::to_document;
use serde_json::{json, Value};
use tauri::{AppHandle, Manager};
// use uuid::Uuid;

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

    let fmt_args = match args.get("account_id") {
        Some(_) => Some(args.to_owned()),
        None => return R::<()>::fail_none(),
    };

    let db = ctx.state::<DB>();

    // db.insert_one(
    //     Entity::Record,
    //     RecordsArg {
    //         _id: None,
    //         scrape_id: Some("dfscxds".to_string()),
    //         protocol: Some("dsasada".to_string()),
    //         url: Some("dsffss".to_string()),
    //         data: Some(RecordArg {
    //             name: Some("fdssaasd".to_string()),
    //             firstname: Some("fn".to_string()),
    //             lastname: Some("dfsjl".to_string()),
    //             linkedin: Some("fdscv".to_string()),
    //             title: Some("titledss".to_string()),
    //             company_name: Some("fcxvsfd".to_string()),
    //             company_twitter: Some("vpcodsx".to_string()),
    //             company_website: Some("cdsds".to_string()),
    //             comapny_linkedin: Some("vpcodsx".to_string()),
    //             company_facebook: Some("vpcodsx".to_string()),
    //             company_location: Some("vpcodsx".to_string()),
    //             email_1: Some("vpcodsx".to_string()),
    //             email_2: Some("vpcodsx".to_string()),
    //             email_3: Some("vpcodsx".to_string()),
    //             employees: Some("vpcodsx".to_string()),
    //             is_verified: Some(true),
    //             industry: Some("vpcodsx".to_string()),
    //             phone: Some("vpcodsx".to_string()),
    //             keywords: Some(vec!["vpcodsx".to_string()]),
    //         }),
    //     },
    // );

    // { id : "66477168cf323e7729dc753f" }
    // Some(doc! {"_id": ObjectId::parse_str("66477168cf323e7729dc753f").unwrap() }),

    println!(
        "{:?}",
        db.find::<RecordsArg>(
            Entity::Record,
            Some(
                doc! {"_id": ObjectId::parse_str("66481e736837c66eb2b741f3".to_string()).unwrap() }
            ),
            // None,
        )
        .unwrap()
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
    // task_id: &Uuid,
    task_id: &String,
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
            // _id: Some(Uuid::new_v4()),
            _id: Some(ObjectId::parse_str("66477168cf323e7729dc753f").unwrap()),
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
