use polodb_core::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::libs::db::entity::EntityTrait;

// https://serde.rs/field-attrs.html

#[derive(Debug, Serialize, Deserialize)]
pub struct Records {
    pub _id: ObjectId,
    pub scrape_id: String,
    pub url: String,
    #[serde(with = "Record")]
    pub data: Record,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(alias = "Name")]
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    pub name: String,
    #[serde(alias = "Firstname")]
    #[serde(rename(serialize = "Firstname", deserialize = "firstname"))]
    pub firstname: String,
    #[serde(alias = "Lastname")]
    #[serde(rename(serialize = "Lastname", deserialize = "lastname"))]
    pub lastname: String,
    #[serde(alias = "Linkedin")]
    #[serde(rename(serialize = "Linkedin", deserialize = "linkedin"))]
    pub linkedin: String,
    #[serde(alias = "Title")]
    #[serde(rename(serialize = "Title", deserialize = "title"))]
    pub title: String,
    #[serde(alias = "Company Name")]
    #[serde(rename(serialize = "Company Name", deserialize = "company_name"))]
    pub company_name: String,
    #[serde(alias = "Company Website")]
    #[serde(rename(serialize = "Company Website", deserialize = "company_website"))]
    pub company_website: String,
    #[serde(alias = "Company Linkedin")]
    #[serde(rename(serialize = "Company Linkedin", deserialize = "company_linkedin"))]
    pub comapny_linkedin: String,
    #[serde(alias = "Company Twitter")]
    #[serde(rename(serialize = "Company Twitter", deserialize = "company_twitter"))]
    pub company_twitter: String,
    #[serde(alias = "Company Facebook")]
    #[serde(rename(serialize = "Company Facebook", deserialize = "company_facebook"))]
    pub company_facebook: String,
    #[serde(alias = "Email")]
    #[serde(rename(serialize = "Email", deserialize = "email"))]
    pub email: String,
    #[serde(alias = "isVerified")]
    #[serde(rename(serialize = "isVerified", deserialize = "is_verified"))]
    pub is_verified: bool,
    #[serde(alias = "Company Location")]
    #[serde(rename(serialize = "Company Location", deserialize = "company_location"))]
    pub company_location: String,
    #[serde(alias = "Employees")]
    #[serde(rename(serialize = "Employees", deserialize = "employees"))]
    pub employees: String,
    #[serde(alias = "Phone")]
    #[serde(rename(serialize = "Phone", deserialize = "phone"))]
    pub phone: String,
    #[serde(alias = "Industry")]
    #[serde(rename(serialize = "Industry", deserialize = "industry"))]
    pub industry: String,
    #[serde(alias = "Keywords")]
    #[serde(rename(serialize = "Keywords", deserialize = "keywords"))]
    pub keywords: Vec<String>,
}

// =============================
// ===========================
// =============================

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordsArg {
    pub _id: Option<ObjectId>,
    pub scrape_id: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub data: Option<RecordArg>,
}

impl EntityTrait for RecordsArg {
    fn is_valid(&mut self) -> bool {
        // if self.protocol.is_none() {
        //     return false;
        // }
        true
    }

    fn fmt_insert(&mut self) {
        if self.protocol.is_none() {
            self.protocol = None;
        }

        if self.url.is_none() {
            self.url = None;
        }

        if self.data.is_none() {
            self.data = None;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordArg {
    #[serde(alias = "Name")]
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    pub name: Option<String>,
    #[serde(alias = "Firstname")]
    #[serde(rename(serialize = "Firstname", deserialize = "firstname"))]
    pub firstname: Option<String>,
    #[serde(alias = "Lastname")]
    #[serde(rename(serialize = "Lastname", deserialize = "lastname"))]
    pub lastname: Option<String>,
    #[serde(alias = "Linkedin")]
    #[serde(rename(serialize = "Linkedin", deserialize = "linkedin"))]
    pub linkedin: Option<String>,
    #[serde(alias = "Title")]
    #[serde(rename(serialize = "Title", deserialize = "title"))]
    pub title: Option<String>,
    #[serde(alias = "Company Name")]
    #[serde(rename(serialize = "Company Name", deserialize = "company_name"))]
    pub company_name: Option<String>,
    #[serde(alias = "Company Website")]
    #[serde(rename(serialize = "Company Website", deserialize = "company_website"))]
    pub company_website: Option<String>,
    #[serde(alias = "Company Linkedin")]
    #[serde(rename(serialize = "Company Linkedin", deserialize = "company_linkedin"))]
    pub comapny_linkedin: Option<String>,
    #[serde(alias = "Company Twitter")]
    #[serde(rename(serialize = "Company Twitter", deserialize = "company_twitter"))]
    pub company_twitter: Option<String>,
    #[serde(alias = "Company Facebook")]
    #[serde(rename(serialize = "Company Facebook", deserialize = "company_facebook"))]
    pub company_facebook: Option<String>,
    // #[serde(alias = "Email")]
    // #[serde(rename(serialize = "Email", deserialize = "email"))]
    // pub email: Option<String>,
    #[serde(alias = "Email #1")]
    #[serde(rename(serialize = "Email #1", deserialize = "email_1"))]
    pub email_1: Option<String>,
    #[serde(alias = "Email #2")]
    #[serde(rename(serialize = "Email #2", deserialize = "email_2"))]
    pub email_2: Option<String>,
    #[serde(alias = "Email #3")]
    #[serde(rename(serialize = "Email #3", deserialize = "email_3"))]
    pub email_3: Option<String>,
    #[serde(alias = "isVerified")]
    #[serde(rename(serialize = "isVerified", deserialize = "is_verified"))]
    pub is_verified: Option<bool>,
    #[serde(alias = "Company Location")]
    #[serde(rename(serialize = "Company Location", deserialize = "company_location"))]
    pub company_location: Option<String>,
    #[serde(alias = "Employees")]
    #[serde(rename(serialize = "Employees", deserialize = "employees"))]
    pub employees: Option<String>,
    #[serde(alias = "Phone")]
    #[serde(rename(serialize = "Phone", deserialize = "phone"))]
    pub phone: Option<String>,
    #[serde(alias = "Industry")]
    #[serde(rename(serialize = "Industry", deserialize = "industry"))]
    pub industry: Option<String>,
    #[serde(alias = "Keywords")]
    #[serde(rename(serialize = "Keywords", deserialize = "keywords"))]
    pub keywords: Option<Vec<String>>,
}

impl EntityTrait for RecordArg {
    fn is_valid(&mut self) -> bool {
        // if self.protocol.is_none() {
        //     return false;
        // }
        true
    }

    fn fmt_insert(&mut self) {
        // if self.protocol.is_none() {
        //     self.protocol = None;
        // }

        // if self.url.is_none() {
        //     self.url = None;
        // }

        // if self.data.is_none() {
        //     self.data = None;
        // }
    }
}
