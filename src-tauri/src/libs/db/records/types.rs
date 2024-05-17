use serde::Serialize;

// export type IRecords = {
//   id: string
//   scrapeID: string
//   url: string
//   data: IRecord
// }

// export type IRecord = {
//   Name: string
//   Firstname: string
//   Lastname: string
//   Linkedin: string
//   Title: string
//   'Company Name': string
//   'Company Website': string
//   'Comapny Linkedin': string
//   'Company Twitter': string
//   'Company Facebook': string
//   Email: string
//   isVerified: boolean
//   'Company Location': string
//   Employees: string
//   Phone: string
//   Industry: string
//   Keywords: string[]
// }

#[derive(Debug, Serialize)]
pub struct Records {
    pub _id: String,
    pub scrape_id: String,
    pub url: String,
    pub data: Record,
}

#[derive(Debug, Serialize)]
pub struct Record {
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    pub name: String,
    #[serde(rename(serialize = "Firstname", deserialize = "firstname"))]
    pub firstname: String,
    #[serde(rename(serialize = "Lastname", deserialize = "lastname"))]
    pub lastname: String,
    #[serde(rename(serialize = "Linkedin", deserialize = "linkedin"))]
    pub linkedin: String,
    #[serde(rename(serialize = "Title", deserialize = "title"))]
    pub title: String,
    #[serde(rename(serialize = "Company Name", deserialize = "company_name"))]
    pub company_name: String,
    #[serde(rename(serialize = "Company Website", deserialize = "company_website"))]
    pub company_website: String,
    #[serde(rename(serialize = "Company Linkedin", deserialize = "company_linkedin"))]
    pub comapny_linkedin: String,
    #[serde(rename(serialize = "Company Twitter", deserialize = "company_twitter"))]
    pub company_twitter: String,
    #[serde(rename(serialize = "Company Facebook", deserialize = "company_facebook"))]
    pub company_facebook: String,
    #[serde(rename(serialize = "Email", deserialize = "email"))]
    pub email: String,
    #[serde(rename(serialize = "isVerified", deserialize = "is_verified"))]
    pub is_verified: bool,
    #[serde(rename(serialize = "Company Location", deserialize = "company_location"))]
    pub company_location: String,
    #[serde(rename(serialize = "Employees", deserialize = "employees"))]
    pub employees: String,
    #[serde(rename(serialize = "Phone", deserialize = "phone"))]
    pub phone: String,
    #[serde(rename(serialize = "Industry", deserialize = "industry"))]
    pub industry: String,
    #[serde(rename(serialize = "Keywords", deserialize = "keywords"))]
    pub keywords: Vec<String>,
}

// =============================
// ===========================
// =============================

pub struct RecordsArg {
    pub _id: Option<String>,
    pub scrape_id: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub data: Option<RecordArg>,
}

impl RecordsArg {
    pub fn is_valid(&mut self) -> bool {
        // if self.protocol.is_none() {
        //     return false;
        // }
        true
    }

    pub fn fmt_insert(&mut self) -> &RecordsArg {
        if self.protocol.is_none() {
            self.protocol = None;
        }

        if self.url.is_none() {
            self.url = None;
        }

        if self.data.is_none() {
            self.data = None;
        }

        self
    }
}

#[derive(Debug, Serialize)]
pub struct RecordArg {
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    pub name: Option<String>,
    #[serde(rename(serialize = "Firstname", deserialize = "firstname"))]
    pub firstname: Option<String>,
    #[serde(rename(serialize = "Lastname", deserialize = "lastname"))]
    pub lastname: Option<String>,
    #[serde(rename(serialize = "Linkedin", deserialize = "linkedin"))]
    pub linkedin: Option<String>,
    #[serde(rename(serialize = "Title", deserialize = "title"))]
    pub title: Option<String>,
    #[serde(rename(serialize = "Company Name", deserialize = "company_name"))]
    pub company_name: Option<String>,
    #[serde(rename(serialize = "Company Website", deserialize = "company_website"))]
    pub company_website: Option<String>,
    #[serde(rename(serialize = "Company Linkedin", deserialize = "company_linkedin"))]
    pub comapny_linkedin: Option<String>,
    #[serde(rename(serialize = "Company Twitter", deserialize = "company_twitter"))]
    pub company_twitter: Option<String>,
    #[serde(rename(serialize = "Company Facebook", deserialize = "company_facebook"))]
    pub company_facebook: Option<String>,
    #[serde(rename(serialize = "Email #1", deserialize = "email_1"))]
    pub email_1: Option<String>,
    #[serde(rename(serialize = "Email #2", deserialize = "email_2"))]
    pub email_2: Option<String>,
    #[serde(rename(serialize = "Email #3", deserialize = "email_3"))]
    pub email_3: Option<String>,
    #[serde(rename(serialize = "isVerified", deserialize = "is_verified"))]
    pub is_verified: Option<bool>,
    #[serde(rename(serialize = "Company Location", deserialize = "company_location"))]
    pub company_location: Option<String>,
    #[serde(rename(serialize = "Employees", deserialize = "employees"))]
    pub employees: Option<String>,
    #[serde(rename(serialize = "Phone", deserialize = "phone"))]
    pub phone: Option<String>,
    #[serde(rename(serialize = "Industry", deserialize = "industry"))]
    pub industry: Option<String>,
    #[serde(rename(serialize = "Keywords", deserialize = "keywords"))]
    pub keywords: Option<Vec<String>>,
}

impl RecordArg {
    pub fn is_valid(&mut self) -> bool {
        // if self.protocol.is_none() {
        //     return false;
        // }
        true
    }

    pub fn fmt_insert(&mut self) -> &RecordArg {
        // if self.protocol.is_none() {
        //     self.protocol = None;
        // }

        // if self.url.is_none() {
        //     self.url = None;
        // }

        // if self.data.is_none() {
        //     self.data = None;
        // }

        self
    }
}
