use dotenv::dotenv;
use std::{
    env,
    error::Error,
    time::{Duration, Instant},
};

mod raw_printer;
use raw_printer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn parse_raw_printer_json() {
        dotenv().ok();

        let url = env::var("PRINTER_ADRESS").unwrap();
        let api_key = env::var("PRINTER_API_KEY").unwrap();

        let mut lotta = PrinterBuilder::new(url, api_key)
            .port(80)
            .auto_refresh(Duration::from_secs(5))
            .build();

        let printer = lotta.get_printer_info().await.unwrap();
    }
}

pub struct PrinterBuilder {
    url: String,
    api_key: String,
    port: Option<u32>,
    auto_refresh: Option<Duration>,
}

pub struct Printer {
    url: String,
    api_key: String,
    port: u32,
    client: reqwest::Client,
    printer: Option<RawPrinter>,
    last_refresh: Option<Instant>,
    auto_refresh: Option<Duration>,
}

impl PrinterBuilder {
    pub fn new(url: String, api_key: String) -> Self {
        Self {
            url,
            api_key,
            port: None,
            auto_refresh: None,
        }
    }

    pub fn port(mut self, port: u32) -> Self {
        self.port = Some(port);
        self
    }

    pub fn auto_refresh(mut self, auto_refresh: Duration) -> Self {
        self.auto_refresh = Some(auto_refresh);
        self
    }

    pub fn build(self) -> Printer {
        let url = self.url;
        let api_key = self.api_key;
        let port = self.port.unwrap_or(80);
        let client = reqwest::Client::new();
        let printer = None;
        let last_refresh = None;
        let auto_refresh = self.auto_refresh;

        Printer {
            url,
            api_key,
            port,
            client,
            printer,
            last_refresh,
            auto_refresh,
        }
    }
}

impl Printer {
    /// Returns the current PrusaLink version in a json format
    pub async fn get_version(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("http://{}/api/version", self.url);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;

        let body = res.text().await?;

        Ok(body)
    }

    pub async fn get_printer_info(&mut self) -> Result<RawPrinter, Box<dyn Error>> {
        let url = format!("http://{}:80/api/printer", self.url);

        let raw_printer_text = self
            .client
            .get(&url)
            .header("X-Api-Key", self.api_key())
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<RawPrinter>(&raw_printer_text)?)
    }

    pub async fn refresh(&mut self) {
        let url = format!("http://{}:80/api/printer", self.url);

        let raw_printer_text = self
            .client
            .get(&url)
            .header("X-Api-Key", self.api_key())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        self.printer = Some(serde_json::from_str::<RawPrinter>(&raw_printer_text).unwrap());
        self.last_refresh = Some(Instant::now());
    }
}

// impl block for minor helper functions
impl Printer {
    /// Returns a reference to the url string
    pub fn url(&self) -> &String {
        &self.url
    }

    /// Returns a reference to the api_key string
    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    /// Changes the APIs url
    pub fn change_url(&mut self, url: String) {
        self.url = url;
    }

    /// changes the APIs api key
    pub fn change_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
}
