use chromiumoxide::cdp::browser_protocol::network::CookieParam;
use polodb_core::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
// use uuid::Uuid;

use crate::libs::db::entity::EntityTrait;

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub _id: ObjectId,
    pub domain: String, // enum Domain
    pub trial_time: Option<u64>,
    pub suspended: bool,
    pub login_type: String, // enum
    pub verified: bool,
    pub email: String,
    pub password: String,
    pub proxy: String,
    pub credits_used: Option<u16>,
    pub credit_limit: Option<u16>,
    pub renewal_date: Option<u64>,
    pub renewal_start_date: Option<u64>,
    pub renewal_end_date: Option<u64>,
    pub trial_days_left: Option<u64>,
    pub last_used: Option<u64>,
    pub cookies: Option<Cookies>,
    pub history: Vec<History>,
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub total_page_scrape: u8,
    pub scrape_time: Option<u64>,
    pub list_name: String,
    pub scrape_id: ObjectId,
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
struct CookieParse {
    key: String,
    value: String,
}

// ===========================
// ==========================
// ===========================

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountArg {
    pub _id: Option<ObjectId>,
    pub domain: Option<String>, // enum Domain
    pub trial_time: Option<u64>,
    pub suspended: Option<bool>,
    pub login_type: Option<String>, // enum
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub proxy: Option<String>,
    pub credits_used: Option<u16>,
    pub credit_limit: Option<u16>,
    pub renewal_date: Option<u64>,
    pub renewal_start_date: Option<u64>,
    pub renewal_end_date: Option<u64>,
    pub trial_days_left: Option<u64>,
    pub last_used: Option<u64>,
    pub cookies: Option<String>,
    pub history: Option<Vec<HistoryArg>>,
}

impl EntityTrait for AccountArg {
    fn is_valid(&mut self) -> bool {
        if self.email.is_none() || self.password.is_none() {
            return false;
        }
        true
    }
    fn fmt_insert(&mut self) {
        // if acc.domain.is_none() {}
        // if acc.account_type.is_none() {
        //     acc.account_type = "free"
        // }

        if self.trial_time.is_none() {
            self.trial_time = None;
        }

        if self.suspended.is_none() {
            self.suspended = Some(false);
        }

        if self.login_type.is_none() {
            self.login_type = Some("default".to_string());
        }

        if self.verified.is_none() {
            self.verified = Some(false);
        }

        if self.proxy.is_none() {
            self.proxy = None;
        }

        if self.credits_used.is_none() {
            self.credits_used = None;
        }

        if self.credit_limit.is_none() {
            self.credit_limit = None;
        }

        if self.renewal_date.is_none() {
            self.renewal_date = None;
        }

        if self.renewal_start_date.is_none() {
            self.renewal_start_date = None;
        }

        if self.renewal_end_date.is_none() {
            self.renewal_end_date = None;
        }

        if self.trial_days_left.is_none() {
            self.trial_days_left = None;
        }

        if self.last_used.is_none() {
            self.last_used = None;
        }

        if self.cookies.is_none() {
            self.cookies = None;
        }

        if self.history.is_none() {
            self.history = Some(vec![]);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryArg {
    pub total_page_scrape: Option<u8>,
    pub scrape_time: Option<u64>,
    pub list_name: Option<String>,
    pub scrape_id: Option<ObjectId>,
}

// macro_rules! impl_drawable {
//     ($t:ty) => {
//         impl EntityTrait for $t {
//             fn is_valid(&mut self) -> bool {
//                 // if self.email.is_none() || self.password.is_none() {
//                 //     return false;
//                 // }
//                 true
//             }
//             fn fmt_insert(&mut self) {
//                 // if acc.domain.is_none() {}
//                 // if acc.account_type.is_none() {
//                 //     acc.account_type = "free"
//                 // }

//                 if self.trial_time.is_none() {
//                     self.trial_time = None;
//                 }

//                 if self.suspended.is_none() {
//                     self.suspended = Some(false);
//                 }

//                 if self.login_type.is_none() {
//                     self.login_type = Some("default".to_string());
//                 }

//                 if self.verified.is_none() {
//                     self.verified = Some(false);
//                 }

//                 if self.proxy.is_none() {
//                     self.proxy = None;
//                 }

//                 if self.credits_used.is_none() {
//                     self.credits_used = None;
//                 }

//                 if self.credit_limit.is_none() {
//                     self.credit_limit = None;
//                 }

//                 if self.renewal_date.is_none() {
//                     self.renewal_date = None;
//                 }

//                 if self.renewal_start_date.is_none() {
//                     self.renewal_start_date = None;
//                 }

//                 if self.renewal_end_date.is_none() {
//                     self.renewal_end_date = None;
//                 }

//                 if self.trial_days_left.is_none() {
//                     self.trial_days_left = None;
//                 }

//                 if self.last_used.is_none() {
//                     self.last_used = None;
//                 }

//                 if self.cookies.is_none() {
//                     self.cookies = None;
//                 }

//                 if self.history.is_none() {
//                     self.history = Some(vec![]);
//                 }
//             }
//         }
//     };
// }
