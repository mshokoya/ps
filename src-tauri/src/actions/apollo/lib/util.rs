use std::time::Duration;

use anyhow::{Error, Result};
use async_std::task;
use chromiumoxide::{cdp::browser_protocol::network::CookieParam, Element, Page};
use futures::TryFutureExt;
use polodb_core::bson::doc;
use tauri::Manager;

use crate::libs::{
    db::{
        accounts::types::{Account, Cookies},
        entity::Entity,
        index::DB,
    },
    taskqueue::types::TaskActionCTX,
};

pub async fn log_into_apollo_then_visit(
    ctx: TaskActionCTX,
    account: Account,
    url: &str,
) -> Result<()> {
    let page = ctx.page.unwrap();

    // == seprate func
    inject_cookies(&page, account.cookies).await?;
    page.goto(url).await?;
    let url = page.url().await?.unwrap();
    if url.contains("#/login") {
        log_into_apollo(ctx, account).await?;
    }
    // ==

    // insert_apollo_cookies(ctx, account).await;

    ctx.handle
        .emit(
            "apollo",
            doc! {"task_id": ctx.task_id, "message": "Logged into apollo"},
        )
        .unwrap();

    page.goto(url).await?.wait_for_navigation_response().await?;

    let url = page.url().await?;

    if url.is_some() && url.unwrap().contains("/#/login") {
        // login_apollo(ctx, account).await;
        ctx.handle
            .emit(
                "apollo",
                doc! {"task_id": ctx.task_id, "message": "Logged into apollo"},
            )
            .unwrap();
        // let cookies = get_cookies().await;
        // ctx.handle.state::<DB>().update_one(
        //     Entity::Account,
        //     doc! {"_id": account._id},
        //     doc! {"cookies": cookies},
        // )
    }

    Ok(())
}

// === global util

pub async fn inject_cookies(page: &Page, cookies: Option<Cookies>) -> Result<()> {
    if cookies.is_some() {
        page.goto("http://www.google.com").await?;
        page.wait_for_navigation_response().await?;
        page.set_cookies(cookies.unwrap().into()).await?;
    }
    Ok(())
}

pub async fn wait_for_selectors(
    page: &Page,
    selector: String,
    mut interval: u8,
    delay_secs: u8,
) -> Result<Vec<Element>> {
    while interval > 0 {
        let el: Option<Vec<Element>> = page.find_elements(selector).await.ok();
        if el.is_some() {
            return el.unwrap();
        }
        interval -= 1;
        task::sleep(Duration::from_secs(delay_secs)).await;
    }

    Error::new("[Error - wait_for_selectors]: Failed to find selectors")
}

pub async fn wait_for_selector(
    page: &Page,
    selector: String,
    mut interval: u8,
    delay_secs: u8,
) -> Result<Element> {
    while interval > 0 {
        println!("func {} - interval {}", "wait_for_selector", interval);
        let el: Option<Element> = page.find_element(selector).await.ok();
        if el.is_some() {
            return el.unwrap();
        }
        interval -= 1;
        task::sleep(Duration::from_secs(delay_secs)).await;
    }

    Error::new("[Error - wait_for_selector]: Failed to find selector")
}
