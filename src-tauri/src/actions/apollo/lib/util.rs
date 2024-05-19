use std::time::Duration;

use anyhow::{anyhow, Result};
use async_std::task;
use chromiumoxide::{Element, Page};
use uuid::Uuid;

use crate::libs::db::accounts::types::Cookies;

// ===== Apollo Error ======
#[derive(Debug)]
pub struct ApolloError {
    task_id: Uuid,
    message: &'static str,
}
impl ApolloError {
    pub fn new(task_id: Uuid, message: &str) -> Self {
        ApolloError { task_id, message }
    }
}

impl std::error::Error for ApolloError {}

impl std::fmt::Display for ApolloError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ApolloError]: {}", self.message)
    }
}
// ==========================

pub async fn inject_cookies(page: &Page, cookies: &Option<Cookies>) -> Result<()> {
    if cookies.is_some() {
        page.goto("http://www.google.com").await?;
        page.wait_for_navigation_response().await?;
        page.set_cookies(cookies.clone().unwrap().into()).await?;
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
        let el: Option<Vec<Element>> = page.find_elements(&selector).await.ok();
        if el.is_some() {
            return Ok(el.unwrap());
        }
        interval -= 1;
        task::sleep(Duration::from_secs(delay_secs.into())).await;
    }

    Err(anyhow!(
        "[Error - wait_for_selectors]: Failed to find selectors",
    ))
}

pub async fn wait_for_selector(
    page: &Page,
    selector: String,
    mut interval: u8,
    delay_secs: u64,
) -> Result<Element> {
    while interval > 0 {
        println!("func {} - interval {}", "wait_for_selector", interval);
        let el: Option<Element> = page.find_element(&selector).await.ok();
        if el.is_some() {
            return Ok(el.unwrap());
        }
        interval -= 1;
        task::sleep(Duration::from_secs(delay_secs)).await;
    }

    Err(anyhow!(
        "[Error - wait_for_selector]: Failed to find selector"
    ))
}
