# Network Monitoring Metrics Explained

This document explains all the metrics and terminology used in the **Traffic Monitor** and **Network Activity** tabs of rs-fairsight.

## 📊 **Core Traffic Metrics**

### **Bytes**
- **Definition**: The amount of data transferred
- **Units**: Displayed in B (bytes), KB (kilobytes), MB (megabytes), GB (gigabytes)
- **Examples**: 
  - `1,024 bytes = 1 KB`
  - `1,048,576 bytes = 1 MB`

### **Incoming Bytes** (↓)
- **Definition**: Data received by your device from the network
- **What it includes**: Downloads, web pages loading, streaming content, email receiving
- **Icon**: Blue arrow pointing down (↓)

### **Outgoing Bytes** (↑)
- **Definition**: Data sent from your device to the network
- **What it includes**: Uploads, web requests, email sending, video calls
- **Icon**: Orange arrow pointing up (↑)

### **Packets**
- **Definition**: Individual units of data sent over the network
- **Explanation**: Data is broken into small "packets" for transmission
- **Relationship**: Multiple packets make up the total bytes transferred

---

## 🔗 **Network Entities**

### **Hosts**
- **Definition**: Individual computers, servers, or devices on the network
- **Identification**: Usually identified by IP address (e.g., `8.8.8.8`, `192.168.1.1`)
- **Examples**:
  - `8.8.8.8` - Google DNS server
  - `140.82.114.21` - GitHub server
  - `192.168.1.1` - Your router
- **Host Information Includes**:
  - **IP Address**: Unique network identifier
  - **Hostname**: Human-readable name (e.g., `dns.google`)
  - **Domain**: Website domain (e.g., `google.com`)
  - **Country**: Geographic location of the server
  - **ASN**: Autonomous System Number (identifies the network provider)

### **Services**
- **Definition**: Specific network protocols and ports used for communication
- **Format**: `Protocol:Port` (e.g., `TCP:443`, `UDP:53`)
- **Common Services**:
  - **TCP:80** - HTTP (web browsing)
  - **TCP:443** - HTTPS (secure web browsing)
  - **UDP:53** - DNS (domain name resolution)
  - **TCP:25** - SMTP (email sending)
  - **TCP:22** - SSH (secure remote access)
  - **UDP:67/68** - DHCP (IP address assignment)

---

## 📅 **Time-Based Metrics**

### **Sessions**
- **Definition**: A continuous period of network monitoring
- **Duration**: Typically 8-second intervals for periodic saves
- **Purpose**: Groups network activity into manageable time chunks
- **Session Data Includes**:
  - Start and end time
  - Total bytes transferred
  - Top hosts and services used
  - Duration of the session

### **Duration**
- **Definition**: Length of time for monitoring or sessions
- **Units**: Displayed in seconds, minutes, or hours
- **Examples**: `8s`, `5min 30s`, `2h 15min`

---

## 📈 **Statistical Metrics**

### **Unique Hosts**
- **Definition**: Count of distinct IP addresses contacted during monitoring
- **Purpose**: Shows diversity of network connections
- **Example**: If you visit Google, Facebook, and GitHub, that's 3 unique hosts

### **Unique Services**
- **Definition**: Count of distinct protocol:port combinations used
- **Purpose**: Shows variety of network services accessed
- **Example**: Web browsing (HTTP/HTTPS) + DNS queries = multiple services

### **Total Sessions**
- **Definition**: Number of monitoring sessions recorded for a time period
- **Daily Context**: Shows how many 8-second intervals were captured
- **High Numbers**: Indicate continuous network activity

---

## 🔄 **Real-time vs Saved Data**

### **Real-time Monitoring Data**
- **Source**: Direct from network adapters (persistent state)
- **Accuracy**: Most current and accurate
- **Updates**: Live, every few seconds
- **Purpose**: Shows actual cumulative network usage

### **Saved Session Data**
- **Source**: Stored session files on disk
- **Purpose**: Historical record and detailed analysis
- **Structure**: Broken into 8-second intervals
- **Note**: May show higher totals due to incremental saves

---

## 🌐 **Network Adapter Information**

### **Adapter Name**
- **Definition**: Technical identifier for network interface
- **Format**: Usually starts with `\Device\NPF_`
- **Examples**: WiFi adapter, Ethernet adapter, VPN adapter

### **Cumulative Totals**
- **Definition**: Total traffic since monitoring started
- **Persistence**: Survives app restarts
- **Reset**: Only when explicitly cleared or system changes

### **Lifetime Stats**
- **Definition**: All-time traffic totals for each adapter
- **Purpose**: Long-term usage tracking
- **Includes**: Total bytes and packets since first use

---

## 📊 **Chart and Graph Metrics**

### **Daily Traffic Charts**
- **X-Axis**: Dates
- **Y-Axis**: Traffic in MB (megabytes)
- **Lines**: Separate lines for incoming (blue) and outgoing (orange) traffic
- **Purpose**: Shows traffic patterns over time

### **Traffic Distribution**
- **Type**: Pie chart
- **Shows**: Proportion of incoming vs outgoing traffic
- **Colors**: Blue for incoming, orange for outgoing

### **Top Hosts List**
- **Ranking**: Sorted by total bytes (incoming + outgoing)
- **Information**: IP, hostname, country, total traffic
- **Purpose**: Identify most-used network destinations

### **Top Services List**
- **Ranking**: Sorted by total bytes used
- **Information**: Protocol, port, service name, traffic
- **Purpose**: Identify most-used network protocols

---

## 🚨 **Status Indicators**

### **Live Data Badge**
- **Green "Live"**: Real-time monitoring is active
- **Purpose**: Confirms current data accuracy

### **Saved Badge**
- **Blue "Saved"**: Data has been stored to disk
- **Purpose**: Confirms data persistence

### **Data Synchronization Notice**
- **Warning**: Appears when real-time and session data differ significantly
- **Reason**: Usually due to multiple overlapping session saves
- **Resolution**: Real-time data is the authoritative source

---

## 🔧 **Technical Details**

### **Packet Capture**
- **Method**: Real packet inspection using network adapter
- **Accuracy**: Counts actual network traffic
- **Fallback**: Simulation mode if packet capture fails

### **Incremental Saves**
- **Frequency**: Every 8 seconds
- **Data**: Only new traffic since last save
- **Purpose**: Prevents data loss during unexpected shutdowns

### **Session Consolidation**
- **Trigger**: When more than 100 sessions exist for one day
- **Method**: Groups sessions into 30-minute time windows
- **Purpose**: Prevents session file bloat while preserving data

---

## 💡 **Understanding Your Data**

### **What Normal Traffic Looks Like**
- **Web Browsing**: Mostly incoming (downloading pages, images)
- **Video Streaming**: High incoming (downloading video data)
- **File Uploads**: High outgoing (sending files to cloud)
- **Video Calls**: Balanced incoming and outgoing

### **Common Hosts You'll See**
- **DNS Servers**: `8.8.8.8`, `1.1.1.1` (for domain name resolution)
- **CDNs**: Cloudflare, Amazon CloudFront (for fast content delivery)
- **Social Media**: Facebook, Twitter, Instagram servers
- **Cloud Services**: Google, Microsoft, Amazon servers

### **When to Investigate**
- **Unusually High Traffic**: May indicate background downloads or updates
- **Unknown Hosts**: Could be legitimate apps or potential security concerns
- **Unexpected Services**: Unusual ports might indicate new software or issues

---

## 📚 **Quick Reference**

| Metric | What It Measures | Good For |
|--------|------------------|----------|
| Total Bytes | Overall data usage | Bandwidth monitoring |
| Unique Hosts | Number of destinations | Network diversity |
| Top Services | Most-used protocols | Understanding app behavior |
| Sessions | Activity periods | Timeline analysis |
| Real-time Data | Current accurate totals | Live monitoring |
| Session Data | Historical details | Detailed analysis |

---

*This documentation covers all metrics displayed in rs-fairsight's network monitoring features. For technical support or questions, refer to the main README.md file.*
