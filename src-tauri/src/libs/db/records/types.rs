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
    _id: String,
    scrape_id: String,
    url: String,
    data: Record,
}

#[derive(Debug, Serialize)]
pub struct Record {
    #[serde(rename(serialize = "Name", deserialize = "name"))]
    name: String,
    #[serde(rename(serialize = "Firstname", deserialize = "firstname"))]
    firstname: String,
    #[serde(rename(serialize = "Lastname", deserialize = "lastname"))]
    lastname: String,
    #[serde(rename(serialize = "Linkedin", deserialize = "linkedin"))]
    linkedin: String,
    #[serde(rename(serialize = "Title", deserialize = "title"))]
    title: String,
    #[serde(rename(serialize = "Company Name", deserialize = "company_name"))]
    company_name: String,
    #[serde(rename(serialize = "Company Website", deserialize = "company_website"))]
    company_website: String,
    #[serde(rename(serialize = "Company Linkedin", deserialize = "company_linkedin"))]
    comapny_linkedin: String,
    #[serde(rename(serialize = "Company Twitter", deserialize = "company_twitter"))]
    company_twitter: String,
    #[serde(rename(serialize = "Company Facebook", deserialize = "company_facebook"))]
    company_facebook: String,
    #[serde(rename(serialize = "Email", deserialize = "email"))]
    email: String,
    #[serde(rename(serialize = "isVerified", deserialize = "is_verified"))]
    is_verified: bool,
    #[serde(rename(serialize = "Company Location", deserialize = "company_location"))]
    company_location: String,
    #[serde(rename(serialize = "Employees", deserialize = "employees"))]
    employees: String,
    #[serde(rename(serialize = "Phone", deserialize = "phone"))]
    phone: String,
    #[serde(rename(serialize = "Industry", deserialize = "industry"))]
    industry: String,
    #[serde(rename(serialize = "Keywords", deserialize = "keywords"))]
    keywords: Vec<String>,
}

// =============================
// ===========================
// =============================

pub struct RecordsArg {
    _id: Option<String>,
    scrape_id: Option<String>,
    protocol: Option<String>,
    url: Option<String>,
    data: Option<RecordArg>,
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
    name: Option<String>,
    #[serde(rename(serialize = "Firstname", deserialize = "firstname"))]
    firstname: Option<String>,
    #[serde(rename(serialize = "Lastname", deserialize = "lastname"))]
    lastname: Option<String>,
    #[serde(rename(serialize = "Linkedin", deserialize = "linkedin"))]
    linkedin: Option<String>,
    #[serde(rename(serialize = "Title", deserialize = "title"))]
    title: Option<String>,
    #[serde(rename(serialize = "Company Name", deserialize = "company_name"))]
    company_name: Option<String>,
    #[serde(rename(serialize = "Company Website", deserialize = "company_website"))]
    company_website: Option<String>,
    #[serde(rename(serialize = "Company Linkedin", deserialize = "company_linkedin"))]
    comapny_linkedin: Option<String>,
    #[serde(rename(serialize = "Company Twitter", deserialize = "company_twitter"))]
    company_twitter: Option<String>,
    #[serde(rename(serialize = "Company Facebook", deserialize = "company_facebook"))]
    company_facebook: Option<String>,
    #[serde(rename(serialize = "Email #1", deserialize = "email_1"))]
    email_1: Option<String>,
    #[serde(rename(serialize = "Email #2", deserialize = "email_2"))]
    email_2: Option<String>,
    #[serde(rename(serialize = "Email #3", deserialize = "email_3"))]
    email_3: Option<String>,
    #[serde(rename(serialize = "isVerified", deserialize = "is_verified"))]
    is_verified: Option<bool>,
    #[serde(rename(serialize = "Company Location", deserialize = "company_location"))]
    company_location: Option<String>,
    #[serde(rename(serialize = "Employees", deserialize = "employees"))]
    employees: Option<String>,
    #[serde(rename(serialize = "Phone", deserialize = "phone"))]
    phone: Option<String>,
    #[serde(rename(serialize = "Industry", deserialize = "industry"))]
    industry: Option<String>,
    #[serde(rename(serialize = "Keywords", deserialize = "keywords"))]
    keywords: Option<Vec<String>>,
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
