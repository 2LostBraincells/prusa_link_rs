use std::collections::HashMap;

use serde::{Serialize, Deserialize};

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
