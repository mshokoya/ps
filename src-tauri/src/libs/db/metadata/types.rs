use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
// use ts_rs::TS;

// export type IMetaData = {
//   id: string
//   url: string
//   params: { [key: string]: string }
//   name: string
//   scrapes: { scrapeID: string; listName: string; length: number; date: number }[]
//   accounts: { accountID: string; range: [min: number, max: number] }[]
// }

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize)]
pub struct Metadata {
    _id: String,
    url: String,
    params: HashMap<String, Value>,
    name: String,
    scrapes: Vec<Scrapes>,
    accounts: Vec<Accounts>,
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize)]
pub struct Accounts {
    account_id: String,
    range: [u32; 2],
}

// #[derive(TS)]
// #[ts(export)]
#[derive(Debug, Serialize)]
pub struct Scrapes {
    scrape_id: String,
    list_name: String,
    length: u8,
    data: u64,
}

// ==================================
// =================================
// ==================================

#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataArg {
    pub _id: Option<String>,
    pub domain: Option<String>,
    verified: Option<bool>,
    mx_records: Option<bool>,
    txt_records: Option<bool>,
    message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountsArg {
    account_id: String,
    range: [u32; 2],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScrapesArg {
    scrape_id: String,
    list_name: String,
    length: u8,
    data: u64,
}

impl MetadataArg {
    pub fn is_valid(&mut self) -> bool {
        if self.domain.is_none() {
            return false;
        }
        true
    }
    pub fn fmt_insert(&mut self) -> &MetadataArg {
        if self.verified.is_none() {
            self.verified = Some(false);
        }

        if self.mx_records.is_none() {
            self.mx_records = Some(false);
        }

        if self.txt_records.is_none() {
            self.txt_records = Some(false);
        }

        if self.message.is_none() {
            self.message = None;
        }

        self
    }
}
