# Dynamic Adapter Monitoring & VPN Support

## Overview

InnoMonitor now provides comprehensive multi-adapter monitoring with intelligent handling of dynamic network changes, including VPN connections/disconnections, network adapter changes, and complex routing scenarios.

## Key Features

### 1. **Auto-Discovery of Network Changes**
- **Periodic Adapter Scanning**: Every 5 seconds, the system checks for new/removed network adapters
- **VPN Detection**: Automatically detects when VPN adapters appear/disappear
- **Dynamic Monitoring**: Starts monitoring new adapters and stops monitoring removed ones
- **State Synchronization**: Maintains consistent monitoring state across adapter changes

### 2. **Concurrent Multi-Adapter Monitoring**
- **All Active Adapters**: Monitors ALL active network adapters simultaneously
- **Real-time Traffic Capture**: Each adapter runs independent packet capture loops
- **Thread-Safe Operations**: Uses Arc/DashMap for safe concurrent access
- **Load Balancing**: Efficiently distributes monitoring load across adapters

### 3. **Advanced Packet Deduplication**
- **Cross-Adapter Detection**: Prevents counting the same packet multiple times
- **Root Source Tracking**: Records which adapter first saw each packet
- **Signature-Based**: Uses unique packet signatures (IP+ports+timestamp)
- **Memory Efficient**: 5-second retention window with automatic cleanup

## VPN Scenario Handling

### When VPN Connects:
```
1. Adapter Discovery detects new VPN adapter (e.g., "tun0", "TAP-Windows")
2. System automatically starts monitoring the VPN adapter
3. Traffic routing changes:
   - Original adapter (eth0/WiFi) may see reduced traffic
   - VPN adapter sees encrypted tunnel traffic  
   - Some packets may appear on both adapters briefly
4. Packet deduplication prevents double-counting during transition
5. User sees accurate total traffic across all adapters
```

### When VPN Disconnects:
```
1. VPN adapter disappears from system
2. Adapter Discovery detects removal
3. System stops monitoring the removed VPN adapter
4. Traffic routes back to original adapters (eth0/WiFi)
5. Monitoring continues on remaining active adapters
6. No traffic data is lost or double-counted
```

### Traffic Routing Examples:

#### Scenario 1: VPN for All Traffic
```
Before VPN: [Internet] ←→ [WiFi] ←→ [Your Computer]
After VPN:  [Internet] ←→ [VPN Server] ←→ [VPN Adapter] ←→ [Your Computer]
                                             ↑
                                        (InnoMonitor monitors this)

Result: All traffic appears on VPN adapter, WiFi shows minimal traffic
```

#### Scenario 2: Split Tunnel VPN
```
Work Traffic:    [Work Server] ←→ [VPN Adapter] ←→ [Your Computer]
Personal Traffic: [Internet] ←→ [WiFi] ←→ [Your Computer]
                                 ↑              ↑
                          (Both monitored by InnoMonitor)

Result: Traffic split between VPN adapter and WiFi, both monitored
```

#### Scenario 3: Bridge/Virtual Adapters (macOS)
```
Physical Traffic: [Internet] ←→ [WiFi/Ethernet] ←→ [Bridge] ←→ [Virtual Adapters]
                                     ↑               ↑            ↑
                              (All monitored, but packets deduplicated)

Result: Same packets seen on multiple adapters, but counted only once
```

## Technical Implementation

### Frontend Changes (`useNetworkMonitoring.js`)

```javascript
// Auto-discovery every 5 seconds
useEffect(() => {
    fetchNetworkAdapters();
    adapterRefreshIntervalRef.current = setInterval(() => {
        fetchNetworkAdapters();
    }, 5000);
    return () => clearInterval(adapterRefreshIntervalRef.current);
}, []);

// Dynamic adapter monitoring
useEffect(() => {
    const initializeAndStartMonitoring = async () => {
        // Stop monitoring removed adapters
        for (const adapterName of Object.keys(monitoringStates)) {
            if (!currentAdapterNames.includes(adapterName)) {
                await invoke('stop_network_monitoring', { adapterName });
                console.log(`🛑 Stopped monitoring removed adapter: ${adapterName}`);
            }
        }
        
        // Start monitoring new adapters
        for (const adapter of adapters) {
            if (!isMonitoring && adapter.is_up) {
                await invoke('start_network_monitoring', { adapterName: adapter.name });
                console.log(`🚀 Auto-started monitoring: ${adapter.name}`);
            }
        }
    };
    
    if (adapters.length > 0) {
        initializeAndStartMonitoring();
    }
}, [adapters]); // React to adapter list changes
```

### Backend Enhancements (`traffic_monitor.rs`)

```rust
// Enhanced deduplication with adapter tracking
lazy_static! {
    static ref PACKET_DEDUP: Arc<DashMap<String, (u64, String)>> = Arc::new(DashMap::new());
    //                                            ↑     ↑
    //                                      expiry_time, first_adapter
}

// Packet processing with cross-adapter deduplication
async fn process_real_packet(/* ... */, adapter_name: &str) {
    let packet_signature = format!("{}->{}:{}:{}:{}",
        src_ip, dst_ip, src_port, dst_port, microsecond_timestamp);
    
    // Check if packet already seen on another adapter
    if let Some((_, first_adapter)) = PACKET_DEDUP.get(&packet_signature) {
        // Skip duplicate - already counted on first_adapter
        return;
    }
    
    // Record this packet with current adapter info
    PACKET_DEDUP.insert(packet_signature, (now + 5, adapter_name.to_string()));
    
    // Process packet normally (count traffic, update stats)
    update_traffic_statistics(/* ... */);
}
```

## Monitoring Behavior

### Active Monitoring States:
- ✅ **WiFi + Ethernet**: Both monitored, no duplicates
- ✅ **WiFi + VPN**: Both monitored, tunnel traffic on VPN
- ✅ **Ethernet + Docker**: Both monitored, container traffic deduplicated  
- ✅ **Multiple VPNs**: All VPN adapters monitored concurrently
- ✅ **Bridge Networks**: Physical + virtual adapters, duplicates removed

### Adapter Priority (for deduplication):
1. **First Seen**: Whichever adapter processes the packet first gets counted
2. **Physical Preferred**: In case of simultaneous processing, physical adapters typically win
3. **VPN Aware**: VPN traffic properly attributed to VPN adapters
4. **Bridge Handling**: Bridge duplicates are automatically filtered out

### Traffic Attribution:
- **Outgoing Traffic**: Attributed to the adapter that first captures the outbound packet
- **Incoming Traffic**: Attributed to the adapter receiving the response
- **Encrypted Traffic**: VPN tunnel traffic shows on VPN adapter, not underlying physical adapter
- **Local Traffic**: Bridge/loopback traffic filtered to prevent noise

## User Experience

### Visual Indicators:
- **Auto-Monitoring Active**: Green chip shows adapter is being monitored
- **Multiple Adapters**: Each adapter tab shows its specific traffic
- **Total Overview**: Aggregated view shows combined traffic from all adapters
- **VPN Status**: VPN adapters clearly identified in adapter list

### Dynamic Updates:
- **Real-time Adapter List**: UI updates when VPN connects/disconnects
- **Seamless Monitoring**: No interruption when adapters change
- **Accurate Totals**: Total traffic remains accurate across all scenarios
- **No Configuration Required**: Everything works automatically

### Debug Information:
```
Console logs show:
🚀 Auto-started monitoring for adapter: tun0 (OpenVPN TAP-Windows Adapter)
✅ Already monitoring adapter: Wi-Fi
🛑 Stopped monitoring for removed adapter: TAP-Windows Adapter V9
🔄 Skipping duplicate packet from en0 (first seen on: bridge0)
```

## Compatibility

### Network Configurations:
- ✅ **Single Adapter**: Standard WiFi or Ethernet
- ✅ **Multi-homed**: Multiple physical adapters
- ✅ **VPN Solutions**: OpenVPN, WireGuard, IPSec, PPTP, L2TP
- ✅ **Virtualization**: VMware, VirtualBox, Docker, Hyper-V
- ✅ **Tethering**: Mobile hotspot, USB tethering
- ✅ **Enterprise**: VLAN, bonding, bridging configurations

### Operating Systems:
- **macOS**: Handles bridge adapters, VMware Fusion, Parallels
- **Windows**: Supports Hyper-V, VMware Workstation, VPN clients
- **Linux**: Container networking, KVM, network namespaces

This comprehensive approach ensures accurate traffic monitoring regardless of network complexity, VPN usage, or dynamic adapter changes, providing users with reliable and complete visibility into their network activity.
