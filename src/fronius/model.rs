use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FroniusBody<D> {
    pub data: D,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct FroniusStatus {
    pub code: u32,
    pub reason: String,
    pub user_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct FroniusHead {
    pub request_arguments: HashMap<String, String>,
    pub status: FroniusStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct FroniusResponse<D> {
    pub body: FroniusBody<D>,
    pub head: FroniusHead,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FroniusValue {
    pub unit: String,
    pub value: Option<f64>,
}

impl fmt::Display for FroniusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(value) => write!(f, "{:.1} {}", value, self.unit),
            None => write!(f, "--- {}", self.unit),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
#[allow(dead_code)]
pub struct FroniusCommonInverterData {
    pub day_energy: FroniusValue,
    pub iac: FroniusValue,
    pub idc: FroniusValue,
    pub idc_2: FroniusValue,
    pub idc_3: FroniusValue,
    pub idc_4: FroniusValue,
    pub pac: FroniusValue,
    pub sac: FroniusValue,
    pub total_energy: FroniusValue,
    pub uac: FroniusValue,
    pub udc: FroniusValue,
    pub udc_2: FroniusValue,
    pub udc_3: FroniusValue,
    pub udc_4: FroniusValue,
    pub year_energy: FroniusValue,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct FroniusSite {
    pub backup_mode: bool,
    pub battery_standby: bool,
    #[serde(rename = "P_Grid")]
    pub power_grid: f64,
    #[serde(rename = "P_Akku")]
    pub power_akku: f64,
    #[serde(rename = "P_Load")]
    pub power_load: f64,
    #[serde(rename = "P_PV")]
    pub power_pv: f64,
    #[serde(rename = "rel_Autonomy")]
    pub rel_autonomy: f64,
    #[serde(rename = "rel_SelfConsumption")]
    pub rel_self_consumption: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FroniusPowerFlowData {
    pub site: FroniusSite,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FroniusStorageControllerData {
    #[serde(rename = "Capacity_Maximum")]
    pub capacity: f64,
    #[serde(rename = "Current_DC")]
    pub current_dc: f64,
    #[serde(rename = "StateOfCharge_Relative")]
    pub rel_charge: f64,
    #[serde(rename = "Voltage_DC")]
    pub voltage_dc: f64,
    #[serde(rename = "Temperature_Cell")]
    pub temperature_cell: f64,
    #[serde(rename = "Enable")]
    pub enable: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FroniusStorageDeviceData {
    pub controller: FroniusStorageControllerData,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct FroniusStorageData(pub HashMap<String, FroniusStorageDeviceData>);

#[derive(Debug, Deserialize, Serialize)]
pub struct FroniusMeterDeviceData {
    #[serde(rename = "PowerReal_P_Sum")]
    pub power_real: f64,
    #[serde(rename = "EnergyReal_WAC_Sum_Consumed")]
    pub energy_real_consumed: f64,
    #[serde(rename = "EnergyReal_WAC_Sum_Produced")]
    pub energy_real_produced: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct FroniusMeterData(pub HashMap<String, FroniusMeterDeviceData>);
