use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
struct CookieParse {
    key: String,
    value: String,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize)]
pub struct Cookies(String);

impl Into<Vec<CookieParam>> for Cookies {
    fn into(self: Self) -> Vec<CookieParam> {
        let fields: Vec<CookieParse> = serde_json::from_str(&self.0).unwrap();
        fields
            .iter()
            .map(|c| CookieParam::new(c.key.to_owned(), c.value.to_owned()))
            .collect::<Vec<CookieParam>>()
    }
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize)]
pub struct History {
    pub total_page_scrape: u8,
    pub scrape_time: Option<u64>,
    pub list_name: String,
    pub scrape_id: Uuid,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub domain: String, // enum Domain
    pub trial_time: Option<u64>,
    pub suspended: bool,
    pub login_type: String, // enum
    pub verified: bool,
    pub email: String,
    pub password: String,
    pub proxy: String,
    pub credits_used: Option<i16>,
    pub credit_limit: Option<i16>,
    pub renewal_date: Option<u64>,
    pub renewal_start_date: Option<u64>,
    pub renewal_end_date: Option<u64>,
    pub trial_days_left: Option<u64>,
    pub last_used: Option<u64>,
    pub cookies: Option<Cookies>,
    pub history: History,
}
