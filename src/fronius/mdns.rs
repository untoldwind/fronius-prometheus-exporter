use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use log::info;
use mdns_sd::{ServiceDaemon, ServiceEvent};

pub fn find_fronius_server() -> Result<String> {
    let mdns = ServiceDaemon::new()?;

    let receiver = mdns.browse("_Fronius-SE-Inverter._tcp.local.")?;

    let timeout = Duration::from_secs(60);
    let now = Instant::now();
    while now.elapsed() < timeout {
        if let ServiceEvent::ServiceResolved(info) = receiver.recv_timeout(timeout)? {
            info!("Resolved: {}", info.fullname);
            for property in info.get_properties().iter() {
                info!("{}: {}", property.key(), property.val_str());
            }
            if let Some(addr) = info.get_addresses_v4().into_iter().next() {
                return Ok(addr.to_string());
            }
        }
    }
    Err(anyhow!("Giving up after {timeout:?}"))
}
