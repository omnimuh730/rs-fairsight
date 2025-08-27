use std::sync::Arc;
use dashmap::DashMap;

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
        ("TCP" | "UDP", 20) => Some("FTP-Data".to_string()),
        ("TCP", 5) => Some("Remote Job Entry".to_string()),
        ("TCP", 7) => Some("Echo".to_string()),
        ("TCP", 9) => Some("Discard".to_string()),
        ("TCP", 11) => Some("Systat".to_string()),
        ("TCP", 13) => Some("Daytime".to_string()),
        ("TCP", 17) => Some("Quote of the Day".to_string()),
        ("TCP", 18) => Some("Message Send Protocol".to_string()),
        ("TCP", 42) => Some("Host Name Server".to_string()),
        ("TCP", 43) => Some("Whois".to_string()),
        ("TCP", 79) => Some("Finger".to_string()),
        ("TCP", 88) => Some("Kerberos".to_string()),
        ("TCP", 109) => Some("POP2".to_string()),
        ("TCP", 113) => Some("Ident".to_string()),
        ("TCP", 119) => Some("NNTP".to_string()),
        ("TCP" | "UDP", 137) => Some("NetBIOS Name Service".to_string()),
        ("TCP" | "UDP", 138) => Some("NetBIOS Datagram Service".to_string()),
        ("TCP" | "UDP", 139) => Some("NetBIOS Session Service".to_string()),
        ("TCP", 161) => Some("SNMP".to_string()), // TCP as well
        ("TCP", 179) => Some("BGP".to_string()),
        ("TCP", 389) => Some("LDAP".to_string()),
        ("TCP", 445) => Some("Microsoft-DS".to_string()),
        ("TCP", 587) => Some("SMTP Submission".to_string()),
        ("TCP", 636) => Some("LDAPS".to_string()),
        ("TCP", 873) => Some("rsync".to_string()),
        ("TCP", 990) => Some("FTPS".to_string()),
        ("TCP", 1723) => Some("PPTP".to_string()),
        ("TCP" | "UDP", 1812) => Some("RADIUS Authentication".to_string()),
        ("TCP" | "UDP", 1813) => Some("RADIUS Accounting".to_string()),
        ("TCP" | "UDP", 4500) => Some("IPsec NAT-T".to_string()),
        ("UDP", 500) => Some("ISAKMP".to_string()),
        ("TCP", 5060) => Some("SIP".to_string()),
        ("UDP", 5060) => Some("SIP".to_string()),
        ("TCP", 5061) => Some("SIPS".to_string()),
        ("TCP" | "UDP", 1080) => Some("Socks Proxy".to_string()),
        ("TCP", 3128) => Some("HTTP Proxy".to_string()),
        ("TCP", 8000) => Some("HTTP-Alt".to_string()), // Often used for HTTP/HTTPS alternatives
        ("TCP", 8080) => Some("HTTP-Alt".to_string()),
        ("TCP", 8443) => Some("HTTPS-Alt".to_string()), // Often used for HTTP/HTTPS alternatives
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
