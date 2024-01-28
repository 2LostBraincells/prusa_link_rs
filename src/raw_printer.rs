use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PrinterTemperature {
    #[serde(rename = "tool0")]
    nozzle: Temp,

    bed: Temp,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temp {
    /// The actual tmperature of the printer
    actual: f32,

    /// The target temperature of the printer
    target: f32,
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
    bed_temp: f32,
    nozzle_temp: f32,
    material: String,
    z_height: f32,
    print_speed: f32,
    axis_x: Option<f32>,
    axis_y: Option<f32>,
    axis_z: Option<f32>,
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

impl RawPrinter {
    pub fn get_paused(&self) -> bool {
        self.state.flags.paused
    }

    pub fn get_operational(&self) -> bool {
        self.state.flags.operational
    }

    pub fn get_ready(&self) -> bool {
        self.state.flags.ready
    }

    pub fn get_sd_ready(&self) -> bool {
        self.state.flags.sd_ready
    }

    pub fn get_error(&self) -> bool {
        self.state.flags.error
    }

    pub fn get_closed_or_error(&self) -> bool {
        self.state.flags.closed_or_error
    }

    pub fn get_finished(&self) -> bool {
        self.state.flags.finished
    }

    pub fn get_prepared(&self) -> bool {
        self.state.flags.prepared
    }

    pub fn get_link_state(&self) -> &str {
        &self.state.flags.link_state
    }

    pub fn get_bed_temp(&self) -> f32 {
        self.telemetry.bed_temp
    }

    pub fn get_target_bed_temp(&self) -> f32 {
        self.temperature.bed.target
    }

    pub fn get_nozzle_temp(&self) -> f32 {
        self.telemetry.nozzle_temp
    }

    pub fn get_target_nozzle_temp(&self) -> f32 {
        self.temperature.nozzle.target
    }

    pub fn get_material_telemetry(&self) -> &str {
        &self.telemetry.material
    }

    pub fn get_z_height_telemetry(&self) -> f32 {
        self.telemetry.z_height
    }

    pub fn get_print_speed_telemetry(&self) -> f32 {
        self.telemetry.print_speed
    }

    pub fn get_axis_x_telemetry(&self) -> Option<f32> {
        self.telemetry.axis_x
    }

    pub fn get_axis_y_telemetry(&self) -> Option<f32> {
        self.telemetry.axis_y
    }
}
