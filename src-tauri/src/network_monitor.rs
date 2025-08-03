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
/// This follows SniffNet's device selection logic to avoid monitoring multiple overlapping interfaces
pub fn get_default_network_adapter() -> Result<String, String> {
    let adapters = get_network_adapters()?;
    
    println!("🔍 Finding best network adapter from {} available adapters", adapters.len());
    
    // Filter and rank adapters by preference (like SniffNet)
    let mut suitable_adapters = Vec::new();
    
    for adapter in &adapters {
        // Skip loopback adapters entirely
        if adapter.is_loopback {
            println!("⏭️  Skipping loopback adapter: {}", adapter.name);
            continue;
        }
        
        // Skip inactive adapters
        if !adapter.is_up {
            println!("⏭️  Skipping inactive adapter: {}", adapter.name);
            continue;
        }
        
        // Calculate preference score
        let mut score = 0i32;
        
        // Prefer adapters with IP addresses
        if !adapter.addresses.is_empty() {
            score += 100;
            println!("✅ Adapter {} has {} addresses (+100 points)", adapter.name, adapter.addresses.len());
        }
        
        // Prefer physical network adapters over virtual ones
        if let Some(ref desc) = adapter.description {
            let desc_lower = desc.to_lowercase();
            
            // Prefer Ethernet and WiFi adapters
            if desc_lower.contains("ethernet") || desc_lower.contains("wifi") || desc_lower.contains("wireless") {
                score += 50;
                println!("✅ Adapter {} is physical network interface (+50 points)", adapter.name);
            }
            
            // Deprioritize virtual adapters commonly found on macOS that cause duplication
            if desc_lower.contains("vmware") || desc_lower.contains("virtualbox") || 
               desc_lower.contains("parallels") || desc_lower.contains("docker") ||
               desc_lower.contains("bridge") || desc_lower.contains("tap") ||
               desc_lower.contains("tun") || desc_lower.contains("vpn") {
                score -= 30;
                println!("⚠️  Adapter {} is virtual/bridge interface (-30 points)", adapter.name);
            }
            
            // Special handling for macOS adapters that might cause duplication
            if desc_lower.contains("en0") || desc_lower.contains("wi-fi") {
                score += 20;  // These are usually the main interfaces on macOS
                println!("✅ Adapter {} appears to be main macOS interface (+20 points)", adapter.name);
            }
        }
        
        // Check for external interfaces (non-private IPs get higher score)
        for addr_str in &adapter.addresses {
            if let Ok(ip) = addr_str.parse::<std::net::IpAddr>() {
                match ip {
                    std::net::IpAddr::V4(ipv4) => {
                        if !ipv4.is_private() && !ipv4.is_loopback() {
                            score += 25;
                            println!("✅ Adapter {} has public IPv4 address (+25 points)", adapter.name);
                            break;
                        }
                    }
                    std::net::IpAddr::V6(_) => {
                        // IPv6 addresses (but not link-local) get moderate preference
                        if !addr_str.starts_with("fe80") {
                            score += 10;
                            println!("✅ Adapter {} has IPv6 address (+10 points)", adapter.name);
                        }
                    }
                }
            }
        }
        
        suitable_adapters.push((adapter, score));
        println!("📊 Adapter {} total score: {}", adapter.name, score);
    }
    
    if suitable_adapters.is_empty() {
        return Err("No suitable network adapters found for monitoring. All adapters are either loopback or inactive.".to_string());
    }
    
    // Sort by score (highest first)
    suitable_adapters.sort_by(|a, b| b.1.cmp(&a.1));
    
    let best_adapter = &suitable_adapters[0].0;
    let best_score = suitable_adapters[0].1;
    
    println!("🎯 Selected best adapter: '{}' (score: {}) - {}", 
        best_adapter.name, 
        best_score,
        best_adapter.description.as_deref().unwrap_or("No description")
    );
    
    Ok(best_adapter.name.clone())
}

/// Get all suitable network adapters for comprehensive monitoring
/// Returns multiple adapters sorted by preference to capture all network traffic
pub fn get_monitoring_adapters() -> Result<Vec<String>, String> {
    let adapters = get_network_adapters()?;
    
    println!("🔍 Finding all suitable adapters for comprehensive monitoring from {} available", adapters.len());
    
    let mut suitable_adapters = Vec::new();
    
    for adapter in &adapters {
        // Skip loopback adapters entirely
        if adapter.is_loopback {
            println!("⏭️  Skipping loopback adapter: {}", adapter.name);
            continue;
        }
        
        // Skip inactive adapters
        if !adapter.is_up {
            println!("⏭️  Skipping inactive adapter: {}", adapter.name);
            continue;
        }
        
        // Calculate preference score
        let mut score = 0i32;
        let mut should_monitor = false;
        
        // Include adapters with IP addresses
        if !adapter.addresses.is_empty() {
            score += 100;
            should_monitor = true;
            println!("✅ Adapter {} has {} addresses (+100 points)", adapter.name, adapter.addresses.len());
        }
        
        // Prefer physical network adapters over virtual ones
        if let Some(ref desc) = adapter.description {
            let desc_lower = desc.to_lowercase();
            
            // Prefer Ethernet and WiFi adapters
            if desc_lower.contains("ethernet") || desc_lower.contains("wifi") || desc_lower.contains("wireless") {
                score += 50;
                should_monitor = true;
                println!("✅ Adapter {} is physical network interface (+50 points)", adapter.name);
            }
            
            // Still include virtual adapters but with lower priority for comprehensive monitoring
            if desc_lower.contains("vmware") || desc_lower.contains("virtualbox") || 
               desc_lower.contains("parallels") || desc_lower.contains("docker") ||
               desc_lower.contains("bridge") || desc_lower.contains("tap") ||
               desc_lower.contains("tun") || desc_lower.contains("vpn") {
                score -= 30;
                // Don't exclude completely - might still carry traffic
                println!("⚠️  Adapter {} is virtual/bridge interface (-30 points, but including for comprehensive monitoring)", adapter.name);
            }
            
            // Special handling for macOS adapters
            if desc_lower.contains("en0") || desc_lower.contains("wi-fi") {
                score += 20;
                should_monitor = true;
                println!("✅ Adapter {} appears to be main macOS interface (+20 points)", adapter.name);
            }
        }
        
        // Check for external interfaces (non-private IPs get higher score)
        for addr_str in &adapter.addresses {
            if let Ok(ip) = addr_str.parse::<std::net::IpAddr>() {
                match ip {
                    std::net::IpAddr::V4(ipv4) => {
                        if !ipv4.is_private() && !ipv4.is_loopback() {
                            score += 25;
                            should_monitor = true;
                            println!("✅ Adapter {} has public IPv4 address (+25 points)", adapter.name);
                            break;
                        }
                    }
                    std::net::IpAddr::V6(_) => {
                        // IPv6 addresses (but not link-local) get moderate preference
                        if !addr_str.starts_with("fe80") {
                            score += 10;
                            should_monitor = true;
                            println!("✅ Adapter {} has IPv6 address (+10 points)", adapter.name);
                        }
                    }
                }
            }
        }
        
        // Include adapter if it has any positive indicators
        if should_monitor || score > 0 {
            suitable_adapters.push((adapter.name.clone(), score));
            println!("📊 Including adapter {} with score: {}", adapter.name, score);
        }
    }
    
    if suitable_adapters.is_empty() {
        return Err("No suitable network adapters found for monitoring. All adapters are either loopback or inactive.".to_string());
    }
    
    // Sort by score (highest first) but include all suitable ones
    suitable_adapters.sort_by(|a, b| b.1.cmp(&a.1));
    
    let adapter_names: Vec<String> = suitable_adapters.into_iter().map(|(name, _)| name).collect();
    
    println!("🎯 Selected {} adapters for comprehensive monitoring: {:?}", 
        adapter_names.len(), 
        adapter_names
    );
    
    Ok(adapter_names)
}
