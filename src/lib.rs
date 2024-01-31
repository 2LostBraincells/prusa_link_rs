use std::{
    error::Error,
    time::{Duration, Instant},
};

pub mod raw_printer;
use raw_printer::*;

/// Builds a Printer struct with the given address and api key
///
/// optional parameters are port and auto_refresh
#[derive(Debug)]
pub struct PrinterBuilder {
    address: String,
    api_key: String,
    port: u32,
    auto_refresh: Option<Duration>,
}

/// Contains all the information about the printer
/// as well as some helper functions to get the information.
#[derive(Debug)]
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
    /// If you want to use a different port than the default port 80, you can use the `port()` function
    ///
    /// Of you want to refresh the printer information manually you can set `auto_refresh` to None.
    /// By default `auto_refresh` is set 2 second, and it is not recommended to set it to a value lower than 1 second
    /// to avoid spamming the printer with requests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use prusa_link_rs::PrinterBuilder;
    /// use std::time::Duration;
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
            port: 80,
            auto_refresh: Some(Duration::from_secs(2)),
        }
    }

    /// Use this function to set a different port than the default port 80
    pub fn port(mut self, port: u32) -> Self {
        self.port = port;
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
        let port = self.port;
        let api_key = self.api_key;
        let client = reqwest::Client::new();
        let printer = None;
        let last_refresh = None;
        let auto_refresh = self.auto_refresh;

        Printer {
            address,
            port,
            api_key,
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
    ///
    /// # Errors
    ///
    /// If the server returns an empty response, the function will return an Err.
    /// This can happen if the server is not running or if the api key is incorrect.
    ///
    /// Remember to check that youre using the right address and port.
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

        if raw_printer_text.trim().is_empty() {
            return Err("Received an empty response from the server".into());
        }

        Ok(serde_json::from_str::<RawPrinter>(&raw_printer_text)?)
    }

    /// Refreshes the internal printer information by sending a request to the printer.
    ///
    /// # Errors
    ///
    /// If the server returns an empty response, the function will return an Err.
    /// This can happen if the server is not running or if the api key is incorrect.
    ///
    /// # Example
    /// ```should_panic
    /// # use prusa_link_rs::PrinterBuilder;
    /// # use std::time::Duration;
    /// # use tokio_test::block_on;
    /// # block_on(async {
    /// let mut printer = PrinterBuilder::new("address".to_string(), "api_key".to_string())
    ///    .build();
    ///
    /// printer.refresh().await.unwrap(); // Errors since this is not a valid address
    /// # })
    pub async fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        let url = format!("http://{}:{}/api/printer", self.address, self.port);

        let raw_printer_text = self
            .client
            .get(&url)
            .header("X-Api-Key", self.api_key())
            .send()
            .await?
            .text()
            .await?;

        if raw_printer_text.trim().is_empty() {
            return Err("Received an empty response from the server".into());
        }

        self.printer = Some(serde_json::from_str::<RawPrinter>(&raw_printer_text)?);
        self.last_refresh = Some(Instant::now());

        Ok(())
    }

    // Get the printer jobs.
    // TODO: Implement this function
    
    // Create a new printer job.
    // TODO: Implement this function

    // Get the printer status.
    // TODO: Implement this function

    // Get the printe storage information.
    // TODO: Implement this function

    // Get the printer files.
    // TODO: Implement this function

    // Get the printer files recursively.
    // TODO: Implement this function

    // Post gcode to the printer.
    // TODO: Implement this function

    // Create files/directories on the printer.
    // TODO: Implement this function

    // Check if file exists on the printer.
    // TODO: Implement this function

    // Print gcode from printer storage.
    // TODO: Implement this function

    // Delete files/directories on the printer.
    // TODO: Implement this function

    /// Returns the current nozzle temperature of the printer as an f32.
    ///
    /// If auto refresh is enabled, the function will refresh the printer information if
    /// the specified time on `last_refresh` has passed, otherwise it will use the cached information.
    ///
    /// If auto refresh is disabled, the function will only refresh the printer information if
    /// there is no cached information.
    ///
    /// # Errors
    ///
    /// If the server returns an empty response, the function will return an Err.
    /// This can happen if the server is not running or if the api key is incorrect.
    ///
    /// # Example
    ///
    /// ```should_panic
    /// # use prusa_link_rs::PrinterBuilder;
    /// # use std::time::Duration;
    /// # use tokio_test::block_on;
    /// # block_on(async {
    /// let mut printer = PrinterBuilder::new("address".to_string(), "api_key".to_string())
    ///     .auto_refresh(Duration::from_secs(5))
    ///     .build();
    ///
    /// let nozzle_temp = printer.get_nozzle_temp().await.unwrap(); // Errors since this is not a valid address
    /// # })
    /// ```
    pub async fn get_nozzle_temp(&mut self) -> Result<f32, Box<dyn Error>> {
        self.refresh_if_necessary().await?;

        let printer = self.printer.as_ref().unwrap();

        Ok(printer.get_nozzle_temp())
    }

    /// Returns the current bed temperature of the printer.
    ///
    /// If `auto_refresh` is Some, the function will refresh the printer information if
    /// the specified time on `last_refresh` has passed, otherwise it will use the cached information.
    ///
    /// If `auto_refresh` is None, the function will only refresh the printer information if
    /// there is no cached information.
    ///
    /// # Errors
    ///
    /// If the server returns an empty response, the function will return an `Err`.
    /// This can happen if the server is not running or if the api key is incorrect.
    ///
    /// # Example
    ///
    /// ```should_panic
    /// # use prusa_link_rs::PrinterBuilder;
    /// # use std::time::Duration;
    /// # use tokio_test::block_on;
    /// # block_on(async {
    /// let mut printer = PrinterBuilder::new("address".to_string(), "api_key".to_string())
    ///     .auto_refresh(Duration::from_secs(5))
    ///     .build();
    ///
    /// let nozzle_temp = printer.get_bed_temp().await.unwrap(); // Errors since this is not a valid address
    /// # })
    /// ```
    pub async fn get_bed_temp(&mut self) -> Result<f32, Box<dyn Error>> {
        self.refresh_if_necessary().await?;

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

    /// Refreshed the printer information if auto_refresh is enabled and the specified time has passed
    /// since the last refresh.
    ///
    /// If auto_refresh is disabled, the function will refresh the printer information if
    /// there is no cached information.
    async fn refresh_if_necessary(&mut self) -> Result<(), Box<dyn Error>> {
        if match (self.last_refresh, self.auto_refresh) {
                (Some(time), Some(duration)) if time.elapsed() > duration => true,
                (None, _) => true,
                _ => false,
            }
        {
            self.refresh().await?;
        }

        Ok(())
    }
}
