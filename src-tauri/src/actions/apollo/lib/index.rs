use chromiumoxide::Page;
use polodb_core::bson::Uuid;
use tauri::Manager;

use crate::{
    actions::apollo::lib::apollo::apollo_default_login,
    libs::{
        db::{accounts::types::Account, index::DB},
        taskqueue::types::TaskActionCTX,
    },
};

pub async fn apollo_login_credits_info(ctx: TaskActionCTX, account: Account) {
    // apollo_login_then_visit(
    //      ctx
    //     account
    // ).await

    // apollo_credits_info(page, task_id).await
}

pub async fn log_into_apollo(ctx: &TaskActionCTX, account: &Account) -> Result<()> {
    match account.login_type.as_str() {
        // "outlook" => apollo_outlook_login(ctx, account).await,
        // "gmail" => apollo_gmail_login(ctx, account).await,
        _ => apollo_default_login(ctx, account).await,
    }

    // save_cookies(ctx.page.unwrap(), account).await

    todo!()
}
