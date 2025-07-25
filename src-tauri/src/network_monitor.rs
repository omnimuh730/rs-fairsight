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
        Err(e) => Err(format!("Failed to list network devices: {}", e)),
    }
}

/// Get the best available network adapter for monitoring (non-loopback, up, with addresses)
pub fn get_default_network_adapter() -> Result<String, String> {
    let adapters = get_network_adapters()?;
    
    // First, try to find a non-loopback adapter that's up and has addresses
    for adapter in &adapters {
        if !adapter.is_loopback && adapter.is_up && !adapter.addresses.is_empty() {
            return Ok(adapter.name.clone());
        }
    }
    
    // If no ideal adapter found, try any non-loopback adapter that's up
    for adapter in &adapters {
        if !adapter.is_loopback && adapter.is_up {
            return Ok(adapter.name.clone());
        }
    }
    
    // If still no adapter found, try any non-loopback adapter
    for adapter in &adapters {
        if !adapter.is_loopback {
            return Ok(adapter.name.clone());
        }
    }
    
    Err("No suitable network adapter found for monitoring".to_string())
}
