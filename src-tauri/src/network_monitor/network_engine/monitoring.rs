use std::sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex};
use std::time::Duration;
use tokio::task;
use rand::Rng;

use super::packet_capture::{open_packet_capture, parse_packet, cleanup_packet_signatures};
use super::types::PacketInfo;
use crate::network_monitor::state_manager::get_state_manager;

pub async fn start_adapter_monitoring(
    adapter_name: String,
    is_active: Arc<AtomicBool>,
    packets_processed: Arc<std::sync::atomic::AtomicU64>,
    bytes_processed: Arc<std::sync::atomic::AtomicU64>,
    is_running: Arc<AtomicBool>,
) -> Result<task::JoinHandle<()>, String> {
    // Open packet capture
    let capture = open_packet_capture(&adapter_name)?;

    let task = task::spawn(async move {
        let cap = Arc::new(Mutex::new(capture));
        let mut stats_update_interval = tokio::time::interval(Duration::from_secs(1));
        let mut cleanup_counter = 0u64;

        println!("📡 Monitoring packets on: {}", adapter_name);

        loop {
            if !is_running.load(Ordering::Relaxed) || !is_active.load(Ordering::Relaxed) {
                break;
            }

            tokio::select! {
                _ = stats_update_interval.tick() => {
                    // Periodic stats update and cleanup
                    cleanup_counter += 1;
                    if cleanup_counter % 30 == 0 { // Every 30 seconds
                        cleanup_packet_signatures();
                    }
                }
                
                packet_result = tokio::task::spawn_blocking({
                    let cap_clone = Arc::clone(&cap);
                    let adapter_name_clone = adapter_name.clone();
                    move || -> Result<Option<PacketInfo>, String> {
                        let mut cap_lock = cap_clone.lock().map_err(|e| format!("Mutex lock error: {}", e))?;
                        match cap_lock.next_packet() {
                            Ok(packet) => {
                                match parse_packet(packet, &adapter_name_clone) {
                                    Ok(Some(packet_info)) => Ok(Some(packet_info)),
                                    Ok(None) => Ok(None), // Duplicate or filtered
                                    Err(e) => {
                                        // Only log parsing errors occasionally
                                        if rand::random::<u8>() % 100 == 0 {
                                            eprintln!("Packet parse error: {}", e);
                                        }
                                        Ok(None)
                                    }
                                }
                            }
                            Err(pcap::Error::TimeoutExpired) => Ok(None),
                            Err(e) => Err(format!("Capture error: {}", e)),
                        }
                    }
                }) => {
                    match packet_result {
                        Ok(Ok(Some(packet_info))) => {
                            // Process valid packet
                            packets_processed.fetch_add(1, Ordering::Relaxed);
                            bytes_processed.fetch_add(packet_info.size_bytes, Ordering::Relaxed);
                            
                            // Update state manager
                            let bytes_in = if packet_info.is_outgoing { 0 } else { packet_info.size_bytes };
                            let bytes_out = if packet_info.is_outgoing { packet_info.size_bytes } else { 0 };
                            let packets_in = if packet_info.is_outgoing { 0 } else { 1 };
                            let packets_out = if packet_info.is_outgoing { 1 } else { 0 };
                            
                            let _ = get_state_manager().update_traffic(
                                &adapter_name,
                                bytes_in,
                                bytes_out,
                                packets_in,
                                packets_out,
                            );
                        }
                        Ok(Ok(None)) => {
                            // Normal case - timeout or duplicate
                            tokio::task::yield_now().await;
                        }
                        Ok(Err(e)) => {
                            eprintln!("⚠️  Packet capture error on {}: {}", adapter_name, e);
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        }
                        Err(e) => {
                            eprintln!("⚠️  Task error on {}: {}", adapter_name, e);
                            break;
                        }
                    }
                }
            }
        }

        println!("📡 Stopped monitoring: {}", adapter_name);
    });

    Ok(task)
}
