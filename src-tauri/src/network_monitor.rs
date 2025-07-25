use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapter {
    pub name: String,
    pub description: Option<String>,
    pub addresses: Vec<String>,
    pub is_up: bool,
    pub is_loopback: bool,
}

impl NetworkAdapter {
    pub fn new(name: String, description: Option<String>, addresses: Vec<String>, is_up: bool, is_loopback: bool) -> Self {
        Self {
            name,
            description,
            addresses,
            is_up,
            is_loopback,
        }
    }
}

pub fn get_network_adapters() -> Result<Vec<NetworkAdapter>, String> {
    match pcap::Device::list() {
        Ok(devices) => {
            let mut adapters = Vec::new();
            
            for device in devices {
                let addresses: Vec<String> = device.addresses
                    .iter()
                    .map(|addr| addr.addr.to_string())
                    .collect();
                
                let is_loopback = device.flags.is_loopback();
                let is_up = device.flags.is_up();
                
                let adapter = NetworkAdapter::new(
                    device.name,
                    device.desc,
                    addresses,
                    is_up,
                    is_loopback,
                );
                
                adapters.push(adapter);
            }
            
            Ok(adapters)
        }
        Err(e) => Err(format!("Failed to get network devices: {}", e)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMonitorConfig {
    pub selected_adapter: Option<String>,
    pub monitoring_enabled: bool,
}

impl Default for NetworkMonitorConfig {
    fn default() -> Self {
        Self {
            selected_adapter: None,
            monitoring_enabled: false,
        }
    }
}
