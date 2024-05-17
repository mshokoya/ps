use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize)]
pub struct Domain {
    pub _id: String,
    pub domain: String,
    verified: bool,
    mx_records: bool,
    txt_records: bool,
    message: String,
}

// ======================
// =======================
// ======================

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainArg {
    pub _id: Option<String>,
    pub domain: Option<String>,
    verified: Option<bool>,
    mx_records: Option<bool>,
    txt_records: Option<bool>,
    message: Option<String>,
}

impl DomainArg {
    pub fn is_valid(&mut self) -> bool {
        if self.domain.is_none() {
            return false;
        }
        true
    }
    pub fn fmt_insert(&mut self) -> &DomainArg {
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
