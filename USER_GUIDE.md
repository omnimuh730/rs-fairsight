# TinkerTicker User Guide

## Welcome to TinkerTicker! 🎯

TinkerTicker is your comprehensive system monitoring companion that provides real-time insights into your network activity and computer usage patterns. This guide will help you get the most out of the application.

## Table of Contents
1. [Getting Started](#getting-started)
2. [Understanding the Interface](#understanding-the-interface)
3. [Network Monitoring](#network-monitoring)
4. [Activity Tracking](#activity-tracking)
5. [Settings & Configuration](#settings--configuration)
6. [Tips & Best Practices](#tips--best-practices)
7. [Troubleshooting](#troubleshooting)
8. [FAQ](#frequently-asked-questions)

---

## Getting Started

### Installation

#### Windows
1. Download the latest `.msi` installer from the releases page
2. Run the installer with administrator privileges
3. Follow the installation wizard
4. Install Npcap when prompted (required for network monitoring)

#### macOS
1. Download the `.dmg` file from the releases page
2. Open the disk image and drag TinkerTicker to Applications
3. Grant network monitoring permissions when first launched
4. Allow through Gatekeeper if prompted

#### Linux
1. Download the `.AppImage` file
2. Make it executable: `chmod +x TinkerTicker.AppImage`
3. Install libpcap: `sudo apt-get install libpcap-dev` (Ubuntu/Debian)
4. Run with: `./TinkerTicker.AppImage`

### First Launch

When you first open TinkerTicker, you'll be guided through a quick setup:

1. **Grant Permissions**: Allow network monitoring access
2. **Select Adapters**: Choose which network interfaces to monitor
3. **Configure Settings**: Set your monitoring preferences
4. **Start Monitoring**: Begin tracking your network and activity

---

## Understanding the Interface

### Main Dashboard

The dashboard is your central hub for monitoring information:

#### 📊 **Quick Stats Panel**
- **Total Traffic**: Today's incoming and outgoing data
- **Active Time**: Hours spent actively using the computer
- **Network Status**: Current monitoring state and adapter health
- **Top Connections**: Most active network destinations

#### 📈 **Real-time Charts**
- **Traffic Graph**: Live visualization of network activity
- **Activity Timeline**: Visual representation of active vs. idle periods
- **Geographic Map**: World map showing connection locations

#### ⚡ **Live Indicators**
- **Upload/Download Speeds**: Current network rates
- **Packet Counters**: Real-time packet statistics
- **Adapter Status**: Health indicators for each network interface

### Navigation

#### 🌐 **Network Monitor**
Detailed network analysis and traffic breakdown

#### ⏱️ **Activity Reports**
Comprehensive time tracking and productivity analysis

#### ⚙️ **Settings**
Configuration options and preferences

#### 📋 **Data Export**
Export your data for external analysis

---

## Network Monitoring

### Understanding Network Traffic

TinkerTicker monitors all network activity on your selected adapters and provides detailed insights:

#### **What Gets Tracked**
- ✅ All incoming and outgoing data packets
- ✅ Source and destination IP addresses
- ✅ Protocols used (HTTP, HTTPS, DNS, etc.)
- ✅ Data volumes and transfer rates
- ✅ Geographic locations of connections
- ❌ **Privacy Note**: Content of communications is never captured

### Network Hosts

#### **Host Information**
Each external server you connect to is tracked with:
- **IP Address**: The server's network identifier
- **Hostname**: Human-readable server name (e.g., google.com)
- **Country**: Geographic location of the server
- **ISP/Organization**: Who owns the server (e.g., Google LLC)
- **Traffic Volume**: How much data exchanged

#### **Understanding Host Types**
- 🌐 **Web Servers**: Websites and web services
- 📧 **Email Servers**: Mail providers like Gmail, Outlook
- ☁️ **Cloud Services**: AWS, Azure, Google Cloud
- 🎮 **Gaming Servers**: Online game platforms
- 📺 **Streaming Services**: Netflix, YouTube, Spotify
- 🔒 **VPN Servers**: When using VPN connections

### Services and Protocols

#### **Common Services Detected**
- **HTTP (Port 80)**: Regular web traffic
- **HTTPS (Port 443)**: Secure web traffic (most websites)
- **DNS (Port 53)**: Domain name lookups
- **Email Protocols**: SMTP, IMAP, POP3
- **File Transfer**: FTP, SFTP
- **Messaging**: Various chat and communication protocols

#### **Reading Traffic Patterns**
- **High HTTPS Traffic**: Normal web browsing, online services
- **DNS Queries**: Normal network operation
- **Large Downloads**: File downloads, software updates, streaming
- **Frequent Small Packets**: Real-time communications, gaming

### VPN Detection

TinkerTicker intelligently handles VPN connections:
- **Automatic Detection**: Recognizes when VPN is active
- **Adapter Switching**: Follows traffic through VPN interfaces
- **Geographic Accuracy**: Shows VPN server location, not masked location
- **Performance Impact**: Minimal overhead during VPN monitoring

---

## Activity Tracking

### How Activity Detection Works

TinkerTicker uses intelligent detection to understand your computer usage:

#### **Active Periods**
Detected when you're:
- 🖱️ Moving the mouse
- ⌨️ Typing on the keyboard
- 🖱️ Clicking or scrolling
- 🎵 Playing audio/video
- 🕹️ Running games or interactive applications

#### **Idle Periods**
Detected when:
- ⏸️ No input for 5+ minutes
- 🔒 Screen is locked
- 💤 Computer is in sleep mode
- 📱 Away from computer

### Activity Reports

#### **Daily Summary**
- **Total Active Time**: Hours actively using the computer
- **Productivity Score**: Based on active vs. idle ratio
- **Peak Activity Hours**: When you're most active
- **Break Patterns**: Frequency and duration of breaks

#### **Weekly Trends**
- **Work Patterns**: Consistency across weekdays
- **Weekend Usage**: Different patterns on weekends
- **Overtime Tracking**: Extended work sessions
- **Recovery Time**: Adequate breaks and rest

#### **Monthly Analysis**
- **Long-term Trends**: Changes in usage patterns over time
- **Productivity Insights**: Best and worst productivity periods
- **Health Indicators**: Screen time and break frequency
- **Goal Tracking**: Progress toward usage goals

### Privacy and Data Security

#### **What's Stored Locally**
- ✅ Timestamps of active/idle periods
- ✅ Duration of sessions
- ✅ Application usage (optional)
- ✅ Network traffic metadata
- ❌ **Never Stored**: Keystrokes, screenshots, personal content

#### **Data Encryption**
- 🔐 All data encrypted with AES-256
- 🔑 Encryption keys stored securely on your device
- 💾 Local storage only - no cloud uploading
- 🗑️ Automatic cleanup of old data

---

## Settings & Configuration

### Network Monitoring Settings

#### **Adapter Selection**
- Choose which network interfaces to monitor
- Enable/disable specific adapters
- VPN adapter auto-detection
- Wireless vs. wired preference

#### **Data Collection**
- **Max Hosts**: Limit tracked external servers (default: 1000)
- **Max Services**: Limit tracked protocols (default: 100)
- **Capture Filter**: Advanced packet filtering (experts only)
- **Geolocation**: Enable/disable geographic lookups

#### **Performance Settings**
- **Buffer Size**: Memory allocated for packet capture
- **Update Frequency**: How often statistics refresh
- **Background Mode**: Monitoring when app is minimized
- **Simulation Mode**: Fallback when hardware capture fails

### Activity Tracking Settings

#### **Detection Sensitivity**
- **Idle Threshold**: Minutes before considering idle (default: 5)
- **Active Threshold**: Minimum activity to count as active
- **Break Detection**: Automatic break period identification
- **Work Hours**: Define your typical working hours

#### **Privacy Controls**
- **Application Tracking**: Monitor which apps you use
- **Detailed Logging**: Enhanced activity detection
- **Screenshot Capture**: Optional periodic screenshots (disabled by default)
- **Data Retention**: How long to keep historical data

### Storage & Backup

#### **Data Location**
- **Windows**: `C:\Users\[Username]\Documents\TinkerTicker`
- **macOS**: `~/Documents/TinkerTicker`
- **Linux**: `~/.local/share/tinkerticker`

#### **Backup Settings**
- **Automatic Backups**: Daily backup creation
- **Backup Retention**: Keep backups for 30 days
- **Export Options**: JSON, CSV formats
- **Cloud Sync**: Coming in future version

---

## Tips & Best Practices

### Getting Accurate Data

#### **Network Monitoring**
1. **Run as Administrator/Root**: Required for full packet capture
2. **Close Other Monitoring Tools**: Avoid conflicts with other network monitors
3. **Monitor All Adapters**: Include VPN and virtual adapters for complete picture
4. **Regular Restarts**: Restart monitoring if you notice gaps in data

#### **Activity Tracking**
1. **Consistent Usage**: Keep TinkerTicker running for accurate patterns
2. **Calibrate Sensitivity**: Adjust idle detection to match your work style
3. **Review Reports Regularly**: Check weekly summaries for insights
4. **Set Realistic Goals**: Use data to set achievable productivity targets

### Optimizing Performance

#### **For Low-Spec Systems**
- Reduce max hosts and services limits
- Increase update intervals
- Enable simulation mode for network monitoring
- Reduce data retention period

#### **For High-Traffic Networks**
- Increase buffer sizes
- Monitor fewer adapters if possible
- Use filters to focus on relevant traffic
- Regular data cleanup

### Data Interpretation

#### **Network Usage Patterns**
- **Morning Spikes**: Email sync, news, social media
- **Work Hours**: Business applications, video calls
- **Evening Peaks**: Streaming, downloads, social media
- **Weekend Patterns**: Different from weekday usage

#### **Activity Insights**
- **Productivity Trends**: Identify your most productive hours
- **Break Patterns**: Ensure adequate rest periods
- **Focus Sessions**: Long periods of uninterrupted activity
- **Distraction Periods**: Frequent short activities

---

## Troubleshooting

### Common Issues

#### **"No Network Data Appearing"**

**Possible Causes & Solutions**:
1. **Permissions**: Ensure TinkerTicker has administrator/root access
2. **Packet Capture Library**: 
   - Windows: Install/reinstall Npcap
   - macOS: Grant Full Disk Access in Security & Privacy
   - Linux: Install libpcap-dev and add user to wireshark group
3. **Firewall/Antivirus**: Add TinkerTicker to exceptions
4. **Adapter Selection**: Verify correct adapters are selected

#### **"Application Won't Start"**

**Solutions**:
1. **Update Dependencies**: Ensure latest Visual C++ Redistributable (Windows)
2. **Check Logs**: Look in the data directory for error logs
3. **Compatibility Mode**: Try running in compatibility mode (Windows)
4. **Fresh Install**: Uninstall completely and reinstall

#### **"High Memory Usage"**

**Normal Behavior**:
- 50-100MB is normal for basic monitoring
- Memory scales with traffic volume and number of hosts
- Temporary spikes during high-traffic periods

**If Excessive**:
1. **Reduce Limits**: Lower max hosts/services in settings
2. **Restart Monitoring**: Stop and start monitoring to clear buffers
3. **Check for Leaks**: Report if memory continuously grows without traffic
4. **Update Application**: Ensure you have the latest version

#### **"Inaccurate Activity Tracking"**

**Calibration Tips**:
1. **Adjust Sensitivity**: Modify idle threshold to match your behavior
2. **Check Detection**: Verify mouse/keyboard detection is working
3. **Application Conflicts**: Close other activity monitors
4. **System Sleep**: Ensure system sleep is properly detected

### Getting Help

#### **Before Reporting Issues**
1. **Check Latest Version**: Update to the newest release
2. **Review Settings**: Verify configuration is correct
3. **Check System Requirements**: Ensure compatibility
4. **Collect Logs**: Note any error messages or unusual behavior

#### **Reporting Bugs**
Include this information:
- Operating system and version
- TinkerTicker version
- Steps to reproduce the issue
- Error messages (if any)
- Log files from the data directory

#### **Feature Requests**
We welcome suggestions! Consider:
- How the feature would benefit users
- Whether it aligns with privacy principles
- If it's technically feasible
- Examples from other applications

---

## Frequently Asked Questions

### General Usage

**Q: Does TinkerTicker slow down my internet?**
A: No, TinkerTicker only monitors traffic passively and doesn't affect your connection speed.

**Q: Can I use TinkerTicker with a VPN?**
A: Yes! TinkerTicker automatically detects and properly monitors VPN connections.

**Q: How much disk space does TinkerTicker use?**
A: Typically 1-5MB per day of monitoring data, with automatic cleanup of old files.

**Q: Can I monitor multiple computers?**
A: Currently, TinkerTicker monitors one computer per installation. Multi-device support is planned for future versions.

### Privacy & Security

**Q: What data does TinkerTicker collect?**
A: Only metadata about network connections and activity timing. No personal content, passwords, or communications are ever captured.

**Q: Is my data sent to the internet?**
A: No, all data stays on your local device. TinkerTicker has no online components or data transmission.

**Q: Can employers see my TinkerTicker data?**
A: Only if they have physical access to your device and your encryption keys. The data is encrypted and stored locally.

**Q: How secure is the stored data?**
A: Data is encrypted with AES-256 encryption and stored locally on your device with secure key management.

### Technical Questions

**Q: Why does TinkerTicker need administrator privileges?**
A: To access low-level network interfaces for packet capture, which requires elevated permissions on all operating systems.

**Q: Can I run TinkerTicker on a server or headless system?**
A: Currently, TinkerTicker requires a graphical interface. A headless version is being considered for future releases.

**Q: Does TinkerTicker work with Docker or virtual machines?**
A: Yes, but you may need to configure network access and permissions appropriately for the virtualized environment.

**Q: Can I automate TinkerTicker or access data programmatically?**
A: Data files are in JSON format and can be accessed programmatically. A formal API is planned for future versions.

### Data Management

**Q: How do I backup my TinkerTicker data?**
A: Data is automatically backed up daily. You can also manually copy the data directory or use the export feature.

**Q: Can I import data from other monitoring tools?**
A: Currently, no direct import is available, but this feature is being considered for future versions.

**Q: How do I delete old data?**
A: Use the data management settings to configure automatic cleanup, or manually delete files from the data directory.

**Q: Can I export data for analysis in Excel or other tools?**
A: Yes, use the export feature to generate CSV files compatible with spreadsheet applications and analysis tools.

---

## Getting More Help

### Resources
- **Documentation**: Comprehensive guides and API documentation
- **Community Forum**: Connect with other users and share tips
- **GitHub Issues**: Report bugs and request features
- **Email Support**: Contact the development team directly

### Stay Updated
- **Release Notes**: Check for new features and improvements
- **Newsletter**: Subscribe for major updates and tips
- **Social Media**: Follow for announcements and community highlights

---

*Thank you for using TinkerTicker! We're committed to providing you with powerful, privacy-focused monitoring tools that help you understand and optimize your digital activities.*

**Version**: 2.0.0 | **Last Updated**: January 2025
