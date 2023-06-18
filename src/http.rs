use std::error::Error;

use reqwest::header::USER_AGENT;

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String, Box<dyn Error>>;
}

pub struct ReqwestClient;

impl HttpClient for ReqwestClient {
    fn get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.43")
            .send()?;

        let body = resp.text()?;

        Ok(body)
    }
}
