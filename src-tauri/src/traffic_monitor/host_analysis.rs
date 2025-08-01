use std::sync::Arc;
use dashmap::DashMap;

use super::types::NetworkHost;
/*
pub async fn process_host_from_packet(
    ip: &IpAddr, 
    bytes: u64, 
    is_outgoing: bool, 
    hosts: &Arc<DashMap<String, NetworkHost>>, 
    now: u64
) {
    // Skip local/loopback addresses for host tracking
    if ip.is_loopback() || 
       (ip.is_ipv4() && ip.to_string().starts_with("192.168.")) ||
       (ip.is_ipv4() && ip.to_string().starts_with("10.")) ||
       (ip.is_ipv4() && ip.to_string().starts_with("172.")) {
        return;
    }

    let ip_str = ip.to_string();
    
    // Check if we already have this host
    let needs_dns_lookup = !hosts.contains_key(&ip_str);
    
    hosts.entry(ip_str.clone()).and_modify(|host| {
        if is_outgoing {
            host.outgoing_bytes += bytes;
            host.outgoing_packets += 1;
        } else {
            host.incoming_bytes += bytes;
            host.incoming_packets += 1;
        }
        host.last_seen = now;
    }).or_insert_with(|| {
        let (incoming_bytes, outgoing_bytes, incoming_packets, outgoing_packets) = 
            if is_outgoing {
                (0, bytes, 0, 1)
            } else {
                (bytes, 0, 1, 0)
            };

        NetworkHost {
            ip: ip_str.clone(),
            hostname: None,
            domain: None,
            country: None,
            country_code: None,
            asn: None,
            incoming_bytes,
            outgoing_bytes,
            incoming_packets,
            outgoing_packets,
            first_seen: now,
            last_seen: now,
        }
    });

    // Perform DNS and GeoIP lookup for new hosts (in background)
    if needs_dns_lookup {
        let hosts_clone = Arc::clone(hosts);
        let ip_clone = *ip;
        let ip_str_clone = ip_str.clone();
        
        tokio::spawn(async move {
            // DNS lookup
            if let Ok(hostname) = lookup_addr(&ip_clone) {
                if let Some(mut host) = hosts_clone.get_mut(&ip_str_clone) {
                    host.hostname = Some(hostname.clone());
                    host.domain = Some(extract_domain_from_hostname(&hostname));
                }
            }

            // GeoIP lookup
            if let Some((country, country_code, asn)) = lookup_geolocation(&ip_clone).await {
                if let Some(mut host) = hosts_clone.get_mut(&ip_str_clone) {
                    if host.country.is_none() {
                        host.country = country;
                    }
                    if host.country_code.is_none() {
                        host.country_code = country_code;
                    }
                    if host.asn.is_none() {
                        host.asn = asn;
                    }
                }
            }
        });
    }
}
*/
/*
pub async fn lookup_geolocation(ip: &IpAddr) -> Option<(Option<String>, Option<String>, Option<String>)> {
    // Basic country mapping based on IP ranges
    // In a full implementation, you would use MaxMind GeoIP2 databases like sniffnet
    
    let ip_str = ip.to_string();
    
    // Google DNS
    if ip_str.starts_with("8.8.8") || ip_str.starts_with("8.8.4") {
        return Some((
            Some("United States".to_string()),
            Some("US".to_string()),
            Some("AS15169 Google LLC".to_string())
        ));
    }
    
    // Cloudflare DNS
    if ip_str.starts_with("1.1.1") || ip_str.starts_with("1.0.0") {
        return Some((
            Some("United States".to_string()),
            Some("US".to_string()),
            Some("AS13335 Cloudflare".to_string())
        ));
    }
    
    // OpenDNS
    if ip_str.starts_with("208.67.222") || ip_str.starts_with("208.67.220") {
        return Some((
            Some("United States".to_string()),
            Some("US".to_string()),
            Some("AS36692 OpenDNS".to_string())
        ));
    }

    // Microsoft IPs
    if ip_str.starts_with("40.") || ip_str.starts_with("52.") || ip_str.starts_with("13.") {
        return Some((
            Some("United States".to_string()),
            Some("US".to_string()),
            Some("AS8075 Microsoft".to_string())
        ));
    }

    // Amazon AWS
    if ip_str.starts_with("54.") || ip_str.starts_with("3.") {
        return Some((
            Some("United States".to_string()),
            Some("US".to_string()),
            Some("AS16509 Amazon".to_string())
        ));
    }

    // European IP ranges (simplified)
    if ip_str.starts_with("185.") || ip_str.starts_with("31.") {
        return Some((
            Some("Germany".to_string()),
            Some("DE".to_string()),
            Some("AS3320 Deutsche Telekom".to_string())
        ));
    }

    // Check for local/private IPs
    if ip.is_loopback() || 
       ip_str.starts_with("192.168.") || 
       ip_str.starts_with("10.") || 
       ip_str.starts_with("172.") {
        return Some((
            Some("Local Network".to_string()),
            Some("XX".to_string()),
            Some("Private".to_string())
        ));
    }

    // For other IPs, try to guess based on common patterns
    let patterns = [
        ("US", "United States", "AS7922 Comcast"),
        ("CA", "Canada", "AS812 Rogers"),
        ("GB", "United Kingdom", "AS2856 BT"),
        ("DE", "Germany", "AS3320 Deutsche Telekom"),
        ("FR", "France", "AS3215 Orange"),
        ("JP", "Japan", "AS2516 KDDI"),
        ("AU", "Australia", "AS1221 Telstra"),
        ("BR", "Brazil", "AS7738 Telecom Brasil"),
    ];

    // Use a simple hash of the IP to pick a pattern (for demo purposes)
    let ip_hash = ip_str.chars().map(|c| c as u32).sum::<u32>() % patterns.len() as u32;
    let (code, country, asn) = patterns[ip_hash as usize];
    
    Some((
        Some(country.to_string()),
        Some(code.to_string()),
        Some(asn.to_string())
    ))
}
*/
/*
pub fn extract_domain_from_hostname(hostname: &str) -> String {
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 2 {
        format!("{}.{}", parts[parts.len()-2], parts[parts.len()-1])
    } else {
        hostname.to_string()
    }
}
*/

pub fn simulate_network_host(hosts: &Arc<DashMap<String, NetworkHost>>, now: u64) {
    use rand::Rng;
    
    let realistic_hosts = [
        ("8.8.8.8", "dns.google", "google.com", "United States", "US", "AS15169 Google LLC"),
        ("1.1.1.1", "one.one.one.one", "cloudflare.com", "United States", "US", "AS13335 Cloudflare"),
        ("208.67.222.222", "resolver1.opendns.com", "opendns.com", "United States", "US", "AS36692 OpenDNS"),
        ("172.217.14.110", "lga25s62-in-f14.1e100.net", "google.com", "United States", "US", "AS15169 Google LLC"),
        ("151.101.193.140", "reddit.map.fastly.net", "fastly.com", "United States", "US", "AS54113 Fastly"),
        ("13.107.42.14", "outlook-namsouth.office365.com", "microsoft.com", "United States", "US", "AS8075 Microsoft"),
        ("52.84.223.104", "server-52-84-223-104.fra50.r.cloudfront.net", "amazonaws.com", "Germany", "DE", "AS16509 Amazon"),
        ("142.250.191.78", "fra16s18-in-f14.1e100.net", "google.com", "Germany", "DE", "AS15169 Google LLC"),
    ];
    
    let mut rng = rand::rng();
    let (ip, hostname, domain, country, country_code, asn) = realistic_hosts[rng.random_range(0..realistic_hosts.len())];
    
    let incoming = rng.random_range(1024..20480) as u64; // 1KB to 20KB per host
    let outgoing = rng.random_range(512..10240) as u64;  // 0.5KB to 10KB per host

    hosts.entry(ip.to_string()).and_modify(|host| {
        host.incoming_bytes += incoming;
        host.outgoing_bytes += outgoing;
        host.incoming_packets += incoming / 1024 + 1;
        host.outgoing_packets += outgoing / 1024 + 1;
        host.last_seen = now;
    }).or_insert(NetworkHost {
        ip: ip.to_string(),
        hostname: Some(hostname.to_string()),
        domain: Some(domain.to_string()),
        country: Some(country.to_string()),
        country_code: Some(country_code.to_string()),
        asn: Some(asn.to_string()),
        incoming_bytes: incoming,
        outgoing_bytes: outgoing,
        incoming_packets: incoming / 1024 + 1,
        outgoing_packets: outgoing / 1024 + 1,
        first_seen: now,
        last_seen: now,
    });
}
