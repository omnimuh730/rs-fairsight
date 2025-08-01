use std::sync::Arc;
use dashmap::DashMap;
use rand::Rng;

use super::types::ServiceInfo;

pub fn process_service_from_packet(protocol: &str, port: u16, bytes: u64, services: &Arc<DashMap<String, ServiceInfo>>) {
    let service_name = get_service_name(protocol, port);
    let key = format!("{}:{}", protocol, port);
    
    services.entry(key.clone()).and_modify(|service| {
        service.bytes += bytes;
        service.packets += 1;
    }).or_insert(ServiceInfo {
        protocol: protocol.to_string(),
        port,
        service_name,
        bytes,
        packets: 1,
    });
}

pub fn get_service_name(protocol: &str, port: u16) -> Option<String> {
    match (protocol, port) {
        ("TCP", 80) => Some("HTTP".to_string()),
        ("TCP", 443) => Some("HTTPS".to_string()),
        ("TCP" | "UDP", 53) => Some("DNS".to_string()),
        ("TCP", 22) => Some("SSH".to_string()),
        ("TCP", 21) => Some("FTP".to_string()),
        ("TCP", 25) => Some("SMTP".to_string()),
        ("TCP", 993) => Some("IMAPS".to_string()),
        ("TCP", 995) => Some("POP3S".to_string()),
        ("UDP", 123) => Some("NTP".to_string()),
        ("TCP", 3389) => Some("RDP".to_string()),
        ("TCP", 23) => Some("Telnet".to_string()),
        ("UDP", 67) => Some("DHCP".to_string()),
        ("UDP", 68) => Some("DHCP".to_string()),
        ("TCP", 110) => Some("POP3".to_string()),
        ("TCP", 143) => Some("IMAP".to_string()),
        ("TCP", 5432) => Some("PostgreSQL".to_string()),
        ("TCP", 3306) => Some("MySQL".to_string()),
        ("TCP", 1433) => Some("MSSQL".to_string()),
        ("TCP", 6379) => Some("Redis".to_string()),
        ("TCP", 27017) => Some("MongoDB".to_string()),
        ("TCP", 1521) => Some("Oracle".to_string()),
        ("TCP", 5984) => Some("CouchDB".to_string()),
        ("UDP", 161) => Some("SNMP".to_string()),
        ("TCP", 8080) => Some("HTTP-Alt".to_string()),
        ("TCP", 8443) => Some("HTTPS-Alt".to_string()),
        _ => None,
    }
}

pub fn simulate_service(services: &Arc<DashMap<String, ServiceInfo>>, bytes: u64) {
    let service_data = [
        ("TCP", 80, "HTTP"),
        ("TCP", 443, "HTTPS"),
        ("TCP", 53, "DNS"),
        ("UDP", 53, "DNS"),
        ("TCP", 22, "SSH"),
        ("TCP", 21, "FTP"),
        ("TCP", 25, "SMTP"),
        ("TCP", 993, "IMAPS"),
        ("TCP", 995, "POP3S"),
        ("UDP", 123, "NTP"),
    ];

    let mut rng = rand::rng();
    let (protocol, port, service_name) = service_data[rng.random_range(0..service_data.len())];
    let key = format!("{}:{}", protocol, port);
    
    services.entry(key.clone()).and_modify(|service| {
        service.bytes += bytes;
        service.packets += bytes / 1024 + 1;
    }).or_insert(ServiceInfo {
        protocol: protocol.to_string(),
        port,
        service_name: Some(service_name.to_string()),
        bytes,
        packets: bytes / 1024 + 1,
    });
}
