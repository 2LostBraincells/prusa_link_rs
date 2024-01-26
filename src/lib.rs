use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    error::Error,
    time::{Duration, Instant},
};

#[cfg(test)]
mod tests {
    use super::*;
}

pub struct Printer {
    url: String,
    api_key: String,
    client: reqwest::Client,
    printer: Option<RawPrinter>,
    last_refresh: Option<Instant>,
    auto_refresh: Option<Duration>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterTemperature {
    /// A hashmap containing the tools and bed of the printer.
    /// Access the bed with "bed" and the first tool with "tool0"
    #[serde(flatten)]
    devices: HashMap<String, PrinterTemperature>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temp {
    /// The actual tmperature of the printer
    actual: f64,

    /// The target temperature of the printer
    target: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterSd {
    ready: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterState {
    text: String,
    flags: PrinterFlags,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterFlags {
    operational: bool,

    paused: bool,

    printing: bool,

    cancelling: bool,

    pausing: bool,

    #[serde(rename = "sdReady")]
    sd_ready: bool,

    error: bool,

    ready: bool,

    #[serde(rename = "closedOrError")]
    closed_or_error: bool,

    finished: bool,

    prepared: bool,

    link_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterTelemetry {
    bed_temp: f64,
    nozzle_temp: f64,
    material: String,
    z_height: f64,
    print_speed: f64,
    axis_x: Option<f64>,
    axis_y: Option<f64>,
    axis_z: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterStorageInfo {
    free_space: u64,
    total_space: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrinterStorage {
    local: Option<PrinterStorageInfo>,
    sd_card: Option<PrinterStorageInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawPrinter {
    temperature: PrinterTemperature,

    sd: PrinterSd,

    state: PrinterState,

    telemetry: PrinterTelemetry,

    storage: PrinterStorage,
}

impl Printer {
    /// Create a new connection to a printer
    ///
    /// ```rust
    /// use prusa_link_rs::Printer;
    ///
    /// let printer = Printer::new(
    ///     "ip_to_printer".to_string(), "api_key".to_string()
    /// );
    ///
    /// assert_eq!(printer.url(), "ip_to_printer");
    /// assert_eq!(printer.api_key(), "api_key");
    /// ```
    pub fn new(url: String, api_key: String) -> Self {
        let client = reqwest::Client::new();
        Self {
            url,
            api_key,
            client,
            printer: None,
            last_refresh: None,
            auto_refresh: None,
        }
    }

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

    pub async fn get_printer(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("http://{}:80/api/printer", self.url);

        Ok("a".to_string())
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
