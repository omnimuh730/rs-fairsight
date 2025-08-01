# InnoMonitor Architecture

## Current System Overview

InnoMonitor is a network and activity monitoring application built with Tauri (Rust backend + React frontend).

### Core Components

#### Backend (Rust)
- **Tauri Framework**: Desktop app framework with IPC communication
- **Network Monitoring**: Packet capture using pcap/winpcap
- **Data Storage**: JSON files with AES encryption
- **Activity Tracking**: Cross-platform user activity monitoring
- **Web Server**: REST API for external access

#### Frontend (React)
- **Dashboard**: Real-time monitoring displays
- **Charts**: Network traffic visualization
- **Controls**: Start/stop monitoring, export data
- **Settings**: Configuration and preferences

### File Structure

#### Main Backend Files
```
src-tauri/src/
├── main.rs              # Application entry point
├── commands.rs          # Tauri command interface
├── traffic_monitor.rs   # Network monitoring core (1000+ lines)
├── network_storage.rs   # Data persistence layer
├── time_tracker.rs      # Activity tracking
├── network_monitor.rs   # Network adapter management
├── health_monitor.rs    # System health tracking
├── encryption.rs        # Data encryption utilities
├── persistent_state.rs  # Application state management
├── web_server.rs        # REST API server
├── hooks.rs             # System event hooks
└── ui_setup.rs          # Window and tray setup
```

#### Frontend Structure
```
src/
├── App.jsx              # Main application component
├── main.jsx             # React entry point
├── components/          # Reusable UI components
└── assets/              # Static assets
```

### Data Flow

#### Network Monitoring
1. **Packet Capture**: Uses pcap to capture network packets
2. **Packet Analysis**: Extracts IP addresses, ports, protocols
3. **Geolocation**: Resolves IP addresses to countries/ASNs
4. **Data Storage**: Saves sessions to encrypted JSON files
5. **Frontend Display**: Updates charts and statistics in real-time

#### Activity Tracking
1. **System Hooks**: Monitors keyboard/mouse activity
2. **Time Tracking**: Records active/idle periods
3. **Data Aggregation**: Summarizes daily activity
4. **Log Storage**: Encrypted activity logs by date

### Key Technologies

#### Network Capture
- **pcap**: Packet capture library (cross-platform)
- **etherparse**: Rust packet parsing library
- **Geolocation**: MaxMind GeoLite2 databases

#### Storage & Security
- **AES-256-GCM**: Data encryption for stored files
- **JSON**: Human-readable data format
- **Backup System**: Automatic daily backups

#### System Integration
- **Cross-Platform**: Windows, macOS, Linux support
- **System Tray**: Background operation
- **Autostart**: Launch on system startup

### Current Technical Challenges

#### Code Organization
- `traffic_monitor.rs` is too large (1000+ lines)
- Mixed concerns in single files
- Tight coupling between components

#### Performance
- Memory usage increases during long monitoring sessions
- Large JSON files impact load times
- Packet processing could be optimized

#### Maintainability
- Limited test coverage
- Complex state management
- Manual error handling patterns

## Implementation Details

### Network Monitoring Process

#### Packet Capture Flow
```rust
// Simplified packet capture flow
fn start_monitoring() -> Result<(), String> {
    let capture = Capture::from_device(adapter)?
        .promisc(true)
        .snaplen(65535)
        .open()?;
    
    loop {
        match capture.next() {
            Ok(packet) => process_packet(packet),
            Err(e) => handle_error(e),
        }
    }
}
```

#### Data Processing Pipeline
1. Raw packet bytes → Parsed headers
2. Extract source/destination IPs and ports
3. Identify protocols (TCP, UDP, etc.)
4. Resolve IP geolocation
5. Update host and service statistics
6. Store in session data structure

### Storage Architecture

#### File Organization
```
Data Storage Structure:
├── network_data/
│   ├── 2025-01-01.json    # Daily network sessions
│   ├── 2025-01-02.json
│   └── backups/
│       ├── 2025-01-01.backup
│       └── 2025-01-02.backup
├── activity_logs/
│   ├── rs-fairsight(2025-01-01).txt
│   └── rs-fairsight(2025-01-02).txt
└── persistent_state.json   # Application state
```

#### Data Structures
```rust
pub struct NetworkSession {
    pub adapter_name: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub total_incoming_bytes: u64,
    pub total_outgoing_bytes: u64,
    pub traffic_data: Vec<TrafficData>,
    pub top_hosts: Vec<NetworkHost>,
    pub top_services: Vec<ServiceInfo>,
}

pub struct NetworkHost {
    pub ip: String,
    pub hostname: Option<String>,
    pub country: Option<String>,
    pub incoming_bytes: u64,
    pub outgoing_bytes: u64,
    pub first_seen: u64,
    pub last_seen: u64,
}
```

### Communication Layer

#### Tauri Commands
Frontend communicates with backend through Tauri commands:

```typescript
// Frontend: Start monitoring
await invoke('start_network_monitoring', { 
  adapterName: 'eth0' 
});

// Frontend: Get current stats
const stats = await invoke('get_network_stats', { 
  adapterName: 'eth0' 
});
```

```rust
// Backend: Command implementations
#[tauri::command]
pub async fn start_network_monitoring(adapter_name: String) -> Result<String, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    monitor.start_monitoring().await
}

#[tauri::command]
pub fn get_network_stats(adapter_name: String) -> Result<MonitoringStats, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    Ok(monitor.get_stats())
}
```

### Security Implementation

#### Data Encryption
- All stored data is encrypted using AES-256-GCM
- Encryption keys derived from system-specific values
- Nonce generated per file for security

#### Access Control
- Network monitoring requires elevated privileges
- File system access restricted to app directories
- No external network connections except for geolocation

### Performance Characteristics

#### Resource Usage
- **Memory**: ~150MB baseline, grows with active sessions
- **CPU**: Low impact during normal operation, spikes during packet processing
- **Disk**: ~10MB per day of network data (compressed)
- **Network**: Local packet capture only, no external traffic

#### Scalability Limits
- **Adapters**: Tested with up to 4 network adapters simultaneously
- **Packets**: Handles ~1000 packets/second effectively
- **Sessions**: Tested with 30+ day continuous monitoring
- **Storage**: Practical limit ~1GB before performance impact

## Development Environment

### Build Requirements
- **Rust**: 1.70+ with Cargo
- **Node.js**: 18+ with npm
- **Tauri CLI**: Latest version
- **Platform Tools**: Visual Studio Build Tools (Windows), Xcode (macOS), build-essential (Linux)

### Development Commands
```bash
# Install dependencies
npm install

# Start development server
cargo tauri dev

# Build for production
cargo tauri build

# Run tests
cargo test
npm test
```

### Debugging
- **Rust Backend**: Use `println!` and `cargo tauri dev` console
- **Frontend**: Browser dev tools in Tauri window
- **Logs**: Application logs stored in system-specific directories
- **Network Issues**: Use simulation mode for testing without packet capture

This architecture document reflects the current implementation as of the latest codebase state.
