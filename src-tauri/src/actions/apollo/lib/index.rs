use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Context, Error, Result};
use async_std::task::sleep;
use chromiumoxide::{cdp::js_protocol::runtime::EvaluateParams, page};
use polodb_core::bson::doc;
use tauri::Manager;

use crate::libs::{db::accounts::types::Account, taskqueue::types::TaskActionCTX};

use super::util::{
    inject_cookies, wait_for_selector, wait_for_selectors, CreditsEval, CreditsInfo,
};

pub async fn log_into_apollo(ctx: &TaskActionCTX, account: &Account) -> Result<()> {
    match account.login_type.as_str() {
        // "outlook" => apollo_outlook_login(ctx, account).await,
        // "gmail" => apollo_gmail_login(ctx, account).await,
        _ => apollo_default_login(ctx, account).await?,
    }

    // save_cookies(ctx.page.unwrap(), account).await

    todo!()
}

pub async fn log_into_apollo_then_visit(
    ctx: &TaskActionCTX,
    account: &Account,
    url: &str,
) -> Result<()> {
    let page = ctx.page.as_ref().unwrap();

    // == seprate func
    inject_cookies(&page, &account.cookies).await?;
    page.goto(url).await?;
    let url = page.url().await?.unwrap();
    if url.contains("#/login") {
        log_into_apollo(ctx, account).await?;
    }

    ctx.handle
        .emit(
            "apollo",
            doc! {"task_id": ctx.task_id, "message": "Logged into apollo"},
        )
        .unwrap();

    page.goto(url).await?.wait_for_navigation_response().await?;

    Ok(())
}

pub async fn apollo_default_login(ctx: &TaskActionCTX, account: &Account) -> Result<()> {
    let page = ctx.page.as_ref().unwrap();
    let login_input_field_selector = String::from(r#"[class="zp_bWS5y zp_J0MYa"]"#);
    let login_button_selector = String::from(r#"[class="zp-button zp_zUY3r zp_H_wRH"]"#);
    let incorrect_logins_selector = String::from(r#"[class="zp_nFR11"]"#);
    let empty_fields_selector = String::from(r#"[class="error-label zp_HeV9x"]"#);

    page.goto("https://app.apollo.io/#/login").await?;

    let submit_button = wait_for_selector(&page, login_button_selector.as_str(), 5, 2)
        .await
        .with_context(|| "[Error - apollo_default_login]: Failed to find login_button_selector")?;

    let login_fields = wait_for_selectors(&page, login_input_field_selector.as_str(), 5, 2)
        .await
        .with_context(|| "[Error - apollo_default_login]: Failed to find login_button_selectors")?;

    if login_fields.len() < 2 {
        return Err(anyhow!(
            "[Error - apollo_default_login]: Failed to find input & password fields"
        ));
    }

    login_fields
        .get(0)
        .unwrap()
        .type_str(&account.email)
        .await?;

    login_fields
        .get(1)
        .unwrap()
        .type_str(&account.password)
        .await?;

    submit_button.click().await?;

    sleep(Duration::from_secs(2)).await;

    if page.find_element(incorrect_logins_selector).await.is_ok() {
        return Err(anyhow!(
            "[Error - apollo_default_login]: incorrect_logins_selector is ok",
        ));
    }

    if page.find_element(empty_fields_selector).await.is_ok() {
        return Err(anyhow!(
            "[Error - apollo_default_login]: empty_fields_selector is ok",
        ));
    }

    sleep(Duration::from_secs(2)).await;

    let url = page.url().await?.unwrap();
    if url.contains("#/login") || url.contains("google.com") || url.contains("microsoftonline.com")
    {
        return Err(anyhow!(
            "[Error - apollo_default_login]: Failed to navigate to dashboard",
        ));
    }

    Ok(())
}

pub async fn apollo_login_credits_info(
    ctx: &TaskActionCTX,
    account: &Account,
) -> Result<CreditsInfo> {
    let page = ctx.page.as_ref().unwrap();
    if wait_for_selector(page, r#"div[class="zp_ajv0U"]"#, 5, 2)
        .await
        .is_err()
    {
        return Err(anyhow!(
            "[Error - apollo_login_credits_info]: Failed to find credits selector",
        ));
    };

    let credits_evaluate: CreditsEval = page
        .evaluate_expression(
            r#"
          const emailCreditInfo: any = document.querySelectorAll('div[class="zp_ajv0U"]')
          if (!emailCreditInfo && emailCreditInfo.length < 2) return null

          const renewalDate = document.querySelector('[class="zp_SJzex"]')
          if (!renewalDate) return null
          if (!renewalDate.lastChild) return null

          const renewalStartEnd = document.querySelector('[class="zp_kQfcf"]')
          if (!renewalStartEnd) return null

          const trialDaysLeft = document.querySelector('[class="zp_EanJu"]')

          return {
            email_credit_info: emailCreditInfo[1].innerText as string,
            renewal_date: renewalDate.lastChild.innerText as string,
            renewal_start_end: renewalStartEnd.innerText as string,
            trial_days_left: trialDaysLeft ? trialDaysLeft.innerText : null
          }
        "#,
        )
        .await?
        .into_value()?;

    println!("CREDITS EVAL");
    println!("{:?}", &credits_evaluate);

    let credits_info: Vec<&str> = credits_evaluate.email_credits_info.split(' ').collect();
    println!("CREDITS Info");
    println!("{:?}", &credits_info);
    let credits_used = credits_info[0].parse::<u16>()?;
    println!("Email email credits used");
    println!("{:?}", &credits_used);
    let credits_limit = credits_info[2].replace(",", "").parse::<u16>()?;
    println!("credits_limit");
    println!("{:?}", &credits_limit);
    let renewal_date = credits_evaluate
        .renewal_date
        .split(':')
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string();
    println!("renewal_date_time");
    println!("{:?}", &renewal_date);
    let renewal_start_end: Vec<String> = credits_evaluate
        .renewal_start_end
        .split('-')
        .map(|s| s.to_owned())
        .collect();
    println!("renewal_start_end");
    println!("{:?}", &renewal_start_end);
    let renewal_start_date: String = renewal_start_end[0].trim().to_owned();
    println!("renewal_start_date");
    println!("{:?}", &renewal_start_date);
    let renewal_end_date: String = renewal_start_end[1].trim().to_owned();
    println!("renewal_end_date");
    println!("{:?}", &renewal_end_date);

    Ok(CreditsInfo {
        credits_used,
        credits_limit,
        renewal_date,
        renewal_start_date,
        renewal_end_date,
        trial_days_left: credits_evaluate.trial_days_left,
    })
}
