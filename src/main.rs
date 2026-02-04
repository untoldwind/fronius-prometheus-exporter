use clap::Parser;
use fronius::FroniusMeterData;
use log::{debug, error};
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::{net::SocketAddrV4, time::Duration};

use crate::fronius::{
    FroniusClient, FroniusCommonInverterData, FroniusPowerFlowData, FroniusStorageData,
};

mod cli;
mod fronius;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = cli::Args::parse();

    let fronius_client = FroniusClient::new(&args.fronius_host, args.fronius_timeout_sec)?;
    PrometheusBuilder::new()
        .with_http_listener(args.metric_bind.parse::<SocketAddrV4>()?)
        .install()?;

    loop {
        debug!("Update fronius metrics");

        match fronius_client.get_power_flow_data().await {
            Ok(power_flow) => update_power_flow(power_flow),
            Err(err) => error!("{}", err),
        };

        match fronius_client.get_storage_data().await {
            Ok(storage) => update_storage(storage),
            Err(err) => error!("{}", err),
        };

        match fronius_client.get_common_inverter_data().await {
            Ok(inverter) => update_inverter(inverter),
            Err(err) => error!("{}", err),
        };

        match fronius_client.get_meter_data().await {
            Ok(meter) => update_meter(meter),
            Err(err) => error!("{}", err),
        }

        tokio::time::sleep(Duration::from_secs(args.fronius_update_sec as u64)).await;
    }
}

fn update_power_flow(power_flow: FroniusPowerFlowData) {
    gauge!("fronius_power_backup_mode").set(power_flow.site.backup_mode as u32);
    gauge!("fronius_power_battery_standby").set(power_flow.site.battery_standby as u32);
    gauge!("fronius_power_pv").set(power_flow.site.power_pv);
    gauge!("fronius_power_akku").set(power_flow.site.power_akku);
    gauge!("fronius_power_grid").set(power_flow.site.power_grid);
    gauge!("fronius_power_load").set(power_flow.site.power_load);
    gauge!("fronius_power_rel_autonomy").set(power_flow.site.rel_autonomy);
}

fn update_storage(storage: FroniusStorageData) {
    for (device_id, device_data) in storage.0 {
        let labels = [("device_id", device_id)];
        gauge!("fronius_storage_capacity", &labels).set(device_data.controller.capacity);
        gauge!("fronius_storage_current_dc", &labels).set(device_data.controller.current_dc);
        gauge!("fronius_storage_rel_charge", &labels).set(device_data.controller.rel_charge);
        gauge!("fronius_storage_voltage_dc", &labels).set(device_data.controller.voltage_dc);
        gauge!("fronius_storage_temperature_cell", &labels)
            .set(device_data.controller.temperature_cell);
        gauge!("fronius_storage_enable", &labels).set(device_data.controller.enable);
    }
}

fn update_inverter(inverter: FroniusCommonInverterData) {
    gauge!("fronius_inverter_total_energy",).set(inverter.total_energy.value.unwrap_or_default());
    gauge!("fronius_inverter_voltage_ac").set(inverter.uac.value.unwrap_or_default());
    gauge!("fronius_inverter_current_ac").set(inverter.iac.value.unwrap_or_default());
    gauge!("fronius_inverter_power_ac").set(inverter.pac.value.unwrap_or_default());
    gauge!("fronius_inverter_voltage_dc", "string" => "1")
        .set(inverter.udc.value.unwrap_or_default());
    gauge!("fronius_inverter_current_dc", "string" => "1")
        .set(inverter.idc.value.unwrap_or_default());
    gauge!("fronius_inverter_voltage_dc", "string" => "2")
        .set(inverter.udc_2.value.unwrap_or_default());
    gauge!("fronius_inverter_current_dc", "string" => "2")
        .set(inverter.idc_2.value.unwrap_or_default());
    gauge!("fronius_inverter_voltage_dc", "string" => "3")
        .set(inverter.udc_3.value.unwrap_or_default());
    gauge!("fronius_inverter_current_dc", "string" => "3")
        .set(inverter.idc_3.value.unwrap_or_default());
    gauge!("fronius_inverter_voltage_dc", "string" => "4")
        .set(inverter.udc_4.value.unwrap_or_default());
    gauge!("fronius_inverter_current_dc", "string" => "4")
        .set(inverter.idc_4.value.unwrap_or_default());
}

fn update_meter(meter: FroniusMeterData) {
    for (device_id, device_data) in meter.0 {
        let labels = [("device_id", device_id)];
        gauge!("fronius_meter_power_real", &labels).set(device_data.power_real);
        gauge!("fronius_meter_energy_real_consumed", &labels).set(device_data.energy_real_consumed);
        gauge!("fronius_meter_energy_real_produced", &labels).set(device_data.energy_real_produced);
    }
}
