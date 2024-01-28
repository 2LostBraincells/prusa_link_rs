use std::{
    error::Error,
    time::{Duration, Instant},
};

mod raw_printer;
use raw_printer::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn parse_raw_printer_json() {
        dotenv().ok();

        let url = env::var("PRINTER_ADRESS").unwrap();
        let api_key = env::var("PRINTER_API_KEY").unwrap();

        let mut lotta = PrinterBuilder::new(url, api_key).build();

        let printer = lotta.get_printer_info().await.unwrap();

        dbg!(printer);
    }
}

/// Builds a Printer struct with the given address and api key
///
/// optional parameters are port and auto_refresh
pub struct PrinterBuilder {
    address: String,
    api_key: String,
    port: Option<u32>,
    auto_refresh: Option<Duration>,
}

/// Contains all the information about the printer
/// as well as some helper functions to get the information.
pub struct Printer {
    address: String,
    api_key: String,
    port: u32,
    client: reqwest::Client,
    printer: Option<RawPrinter>,
    last_refresh: Option<Instant>,
    auto_refresh: Option<Duration>,
}

impl PrinterBuilder {
    /// Creates a new PrinterBuilder with the given address and api key
    ///
    /// If you want to use a different port than the default port 80, you can use the port() function
    ///
    /// You should set the auto refresh time with auto_refresh() to a value greater than 0 for simplicity, but if you
    /// want to refresh the printer manually, you can leave it at None
    ///
    /// # Example
    ///
    /// ```rust
    /// use prusa_link::PrinterBuilder;
    ///
    /// let printer_builder = PrinterBuilder::new("address".to_string(), "api_key".to_string())
    ///    .port(80)
    ///    .auto_refresh(Duration::from_secs(5));
    ///
    /// let printer = printer_builder.build();
    /// ```
    pub fn new(address: String, api_key: String) -> Self {
        Self {
            address,
            api_key,
            port: None,
            auto_refresh: None,
        }
    }

    /// Use this function to set a different port than the default port 80
    pub fn port(mut self, port: u32) -> Self {
        self.port = Some(port);
        self
    }

    /// Use this function to set the auto refresh time
    /// The auto refresh time is the time after which the next call to get printer information,
    /// such as nozzle temperature, will automatically refresh the printer information.
    ///
    /// If you don't want to use auto refresh, you can leave it at None.
    /// You should set the auto refresh time to a value greater than 1 second to avoid spamming the printer with requests.
    pub fn auto_refresh(mut self, auto_refresh: Duration) -> Self {
        self.auto_refresh = Some(auto_refresh);
        self
    }

    /// Builds the Printer struct
    pub fn build(self) -> Printer {
        let address = self.address;
        let api_key = self.api_key;
        let port = self.port.unwrap_or(80);
        let client = reqwest::Client::new();
        let printer = None;
        let last_refresh = None;
        let auto_refresh = self.auto_refresh;

        Printer {
            address,
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
        let url = format!("http://{}:{}/api/version", self.address, self.port);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;

        let body = res.text().await?;

        Ok(body)
    }

    /// Returns a RawPrinter struct with all the information about the printer
    pub async fn get_printer_info(&mut self) -> Result<RawPrinter, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/printer", self.address, self.port);

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

    /// Refreshes the internal printer information by sending a request to the printer.
    pub async fn refresh(&mut self) {
        let url = format!("http://{}:{}/api/printer", self.address, self.port);

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

    /// Returns the current nozzle temperature of the printer as an f32. 
    ///
    /// If auto refresh is enabled, the function will refresh the printer information if 
    /// the specified time on last_refresh has passed, otherwise it will use the cached information.
    ///
    /// If auto refresh is disabled, the function will only refresh the printer information if
    /// there is no cached information.
    pub async fn get_nozzle_temp(&mut self) -> Result<f32, Box<dyn Error>> {
        if self.last_refresh.is_none()
            || (self.auto_refresh.is_some()
                && self.last_refresh.unwrap().elapsed() > self.auto_refresh.unwrap())
        {
            self.refresh().await;
        }

        let printer = self.printer.as_ref().unwrap();

        Ok(printer.get_nozzle_temp())
    }

    /// Returns the current bed temperature of the printer as an f32. 
    ///
    /// If auto refresh is enabled, the function will refresh the printer information if 
    /// the specified time on last_refresh has passed, otherwise it will use the cached information.
    ///
    /// If auto refresh is disabled, the function will only refresh the printer information if
    /// there is no cached information.
    pub async fn get_bed_temp(&mut self) -> Result<f32, Box<dyn Error>> {
        if self.last_refresh.is_none()
            || (self.auto_refresh.is_some()
                && self.last_refresh.unwrap().elapsed() > self.auto_refresh.unwrap())
        {
            self.refresh().await;
        }

        let printer = self.printer.as_ref().unwrap();

        Ok(printer.get_bed_temp())
    }
}

// impl block for minor helper functions
impl Printer {
    /// Returns a reference to the address string
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns a reference to the api_key string
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Changes the APIs url
    pub fn change_address(&mut self, address: String) {
        self.address = address;
    }

    /// changes the APIs api key
    pub fn change_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
}
