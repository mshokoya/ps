use std::time::Duration;

use anyhow::{Context, Error, Result};
use async_std::task::sleep;

use crate::libs::{db::accounts::types::Account, taskqueue::types::TaskActionCTX};

use super::util::{wait_for_selector, wait_for_selectors};

pub async fn apollo_default_login(ctx: &TaskActionCTX, account: Account) -> Result<()> {
    let page = ctx.page.unwrap();
    let login_input_field_selector = String::from(r#"[class="zp_bWS5y zp_J0MYa"]"#);
    let login_button_selector = String::from(r#"[class="zp-button zp_zUY3r zp_H_wRH"]"#);
    let incorrect_logins_selector = String::from(r#"[class="zp_nFR11"]"#);
    let empty_fields_selector = String::from(r#"[class="error-label zp_HeV9x"]"#);

    page.goto("https://app.apollo.io/#/login").await?;

    let submit_button = wait_for_selector(&page, login_button_selector, 5, 2)
        .await
        .with_context(|| "[Error - apollo_default_login]: Failed to find login_button_selector")?;

    let login_fields = wait_for_selectors(&page, login_input_field_selector, 5, 2)
        .await
        .with_context(|| "[Error - apollo_default_login]: Failed to find login_button_selectors")?;

    if login_fields.len() < 2 {
        return Err(Error::new(
            "[Error - apollo_default_login]: Failed to find input & password fields",
        ));
    }

    login_fields.get(0).unwrap().type_str(account.email).await?;
    login_fields
        .get(1)
        .unwrap()
        .type_str(account.password)
        .await?;

    submit_button.click().await?;

    sleep(Duration::from_secs(2)).await;

    if page.find_element(incorrect_logins_selector).await.is_ok() {
        return Err(Error::new(
            "[Error - apollo_default_login]: incorrect_logins_selector is ok",
        ));
    }

    if page.find_element(empty_fields_selector).await.is_ok() {
        return Err(Error::new(
            "[Error - apollo_default_login]: empty_fields_selector is ok",
        ));
    }

    sleep(Duration::from_secs(2)).await;

    let url = page.url().await?.unwrap();
    if url.contains("#/login") || url.contains("google.com") || url.contains("microsoftonline.com")
    {
        return Err(Error::new(
            "[Error - apollo_default_login]: Failed to navigate to dashboard",
        ));
    }

    Ok(())
}
