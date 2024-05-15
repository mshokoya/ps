use chromiumoxide::Browser;

pub struct Scraper {
    pub browser: Browser,
}

impl Scraper {
    pub fn new(browser: Browser) -> Self {
        Self { browser }
    }
}
