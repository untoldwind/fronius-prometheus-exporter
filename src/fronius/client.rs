use std::time::Duration;

use super::model::{
    FroniusCommonInverterData, FroniusPowerFlowData, FroniusResponse, FroniusStorageData,
};
use anyhow::Result;
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;

pub struct FroniusClient {
    client: Client,
    host: String,
}

impl FroniusClient {
    pub fn new(host: &str, fronius_timeout_sec: u32) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(fronius_timeout_sec as u64))
            .build()?;

        Ok(FroniusClient {
            client,
            host: host.to_string(),
        })
    }

    async fn execute_request<D: DeserializeOwned>(&self, uri: &str) -> Result<D> {
        let response = self
            .client
            .request(Method::GET, format!("http://{}{}", &self.host, uri))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<FroniusResponse<D>>().await?.body.data),
            status => Err(anyhow::format_err!(
                "Fronius: {} {}",
                status.as_u16(),
                status.to_string()
            )),
        }
    }

    pub async fn get_common_inverter_data(&self) -> Result<FroniusCommonInverterData> {
        self.execute_request(
            "/solar_api/v1/GetInverterRealtimeData.cgi?Scope=Device&DataCollection=CommonInverterData",
        ).await
    }

    pub async fn get_power_flow_data(&self) -> Result<FroniusPowerFlowData> {
        self.execute_request("/solar_api/v1/GetPowerFlowRealtimeData.fcgi")
            .await
    }

    pub async fn get_storage_data(&self) -> Result<FroniusStorageData> {
        self.execute_request("/solar_api/v1/GetStorageRealtimeData.cgi")
            .await
    }
}
