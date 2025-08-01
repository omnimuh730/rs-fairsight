# InnoMonitor v1.1.0

> **Advanced Cross-Platform Network Traffic and Activity Monitoring System**

A powerful, real-time network monitoring and activity tracking application built with Tauri (Rust) and React. InnoMonitor provides comprehensive packet-level network analysis, activity monitoring, and data insights with enterprise-grade security and performance.

[![Version](https://img.shields.io/badge/version-v1.1.0-blue.svg)](https://github.com/omnimuh730/rs-fairsight/releases)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.0-blueviolet.svg)](https://tauri.app/)
[![React](https://img.shields.io/badge/react-18.3-blue.svg)](https://reactjs.org/)

## 🚀 Key Features

### 🌐 Advanced Network Monitoring
- **Real-time Packet Capture**: Live packet inspection with libpcap integration
- **Intelligent Traffic Analysis**: Deep packet inspection with protocol identification
- **Host Analysis & Geolocation**: DNS resolution, ASN lookup, and geographic mapping
- **Service Detection**: Automatic identification of network services and protocols
- **Packet Deduplication**: Advanced algorithms to prevent duplicate traffic counting
- **Adapter Management**: Dynamic network adapter monitoring with automatic reconnection

### 📊 Activity & Analytics
- **Real-time Activity Monitoring**: Track computer usage and productivity patterns
- **Weekly Network Analytics**: Comprehensive network activity reports with detailed charts
- **Health Monitoring System**: Intelligent warnings and insights about work habits
- **Beautiful Dashboard**: Material-UI based interface with interactive charts

### 🔒 Security & Privacy
- **Encrypted Local Storage**: Ring encryption for sensitive data
- **Admin Privilege Detection**: Secure packet capture with proper permission handling
- **Data Integrity**: Atomic operations and data validation
- **Privacy-First**: All data processing happens locally

### 🛠 System Integration
- **Cross-Platform Support**: Windows, macOS, and Linux compatibility
- **System Tray Integration**: Background monitoring with minimal resource usage
- **Auto-Startup**: Seamless system integration
- **Backup & Recovery**: Robust data management with daily cleanup

## 📖 Documentation

### 📋 Quick Start
- **[User Guide](./docs/guides/USER_GUIDE.md)** - Complete setup and usage instructions
- **[Installation](./docs/guides/USER_GUIDE.md#installation)** - System requirements and installation steps

### 🏗 Architecture & Development
- **[Architecture Overview](./docs/architecture/ARCHITECTURE.md)** - System design and component architecture
- **[Network Implementation](./docs/architecture/NETWORK_MONITOR_IMPLEMENTATION.md)** - Deep dive into network monitoring
- **[Network Metrics](./docs/architecture/NETWORK_METRICS_EXPLAINED.md)** - Understanding traffic analysis metrics
- **[Packet Deduplication](./docs/architecture/PACKET_DEDUPLICATION_LOGIC.md)** - Advanced deduplication algorithms

### 🔄 Development History
- **[Version Evolution](./docs/VERSION_EVOLUTION.md)** - Complete development timeline and milestones
- **[Changelog](./CHANGELOG.md)** - Detailed version changes and improvements
- **[Refactoring Summary](./docs/development/REFACTORING_SUMMARY.md)** - Major code refactoring efforts
- **[Traffic Monitor Refactoring](./docs/development/TRAFFIC_MONITOR_REFACTORING_SUMMARY.md)** - Modular architecture evolution
- **[Issue Resolution](./docs/development/ISSUE_RESOLUTION_SUMMARY.md)** - Problem-solving documentation

### 🔧 Technical Documentation
- **[Real vs Simulation Analysis](./docs/development/REAL_VS_SIMULATION_ANALYSIS.md)** - Data authenticity and testing
- **[macOS Compatibility](./docs/development/MACOS_FIX_SUMMARY.md)** - Platform-specific optimizations
- **[Dynamic Adapter Monitoring](./docs/development/DYNAMIC_ADAPTER_MONITORING.md)** - Network adapter management
- **[Backup Improvements](./docs/development/BACKUP_IMPROVEMENTS.md)** - Data reliability enhancements

## 🏷 Version History

| Version | Release Date | Key Features | Commit |
|---------|-------------|--------------|---------|
| **v1.1.0** | Aug 2025 | GitHub Actions CI/CD, Documentation Organization | `ba748582` |
| **v1.0.0** | Jul 2025 | Complete Network Monitoring, Production Ready | `86d772ce` |
| **v0.9.0** | Jul 2025 | Advanced Traffic Analysis, Packet Deduplication | `d3a20af4` |
| **v0.5.0** | Jun 2025 | Real-time Packet Capture, Host Analysis | `e35e7491` |
| **v0.2.5** | May 2025 | Basic Activity Monitoring, Initial Release | `1f699f54` |

## 🛠 Technology Stack

**Backend (Rust)**
- **Tauri 2.0** - Cross-platform app framework
- **pcap 2.3** - Network packet capture
- **etherparse 0.15** - Packet parsing and analysis
- **dns-lookup 2.0** - DNS resolution
- **ring 0.17** - Cryptographic operations
- **tokio 1.44** - Async runtime
- **dashmap 6.1** - Concurrent hash maps

**Frontend (React)**
- **React 18.3** - UI framework
- **Material-UI 7.0** - Design system
- **MUI X-Charts** - Data visualization
- **React Router DOM 7.2** - Navigation
- **Day.js & date-fns** - Date management

**Build & DevOps**
- **Vite 6.2** - Build tool
- **GitHub Actions** - CI/CD pipeline
- **Cross-platform builds** - Windows, macOS, Linux

## 🚦 Quick Start

### Prerequisites
- **Node.js 20+** and npm
- **Rust 1.70+** and Cargo
- **Admin privileges** (for packet capture)

### Installation

```bash
# Clone the repository
git clone https://github.com/omnimuh730/rs-fairsight.git
cd rs-fairsight

# Install dependencies
npm install

# Build the application
npm run build

# Run in development
npm run tauri dev

# Build for production
npm run tauri build
```

### First Run
1. **Grant Administrator Privileges** - Required for network packet capture
2. **Select Network Adapter** - Choose your primary network interface
3. **Start Monitoring** - Begin real-time traffic analysis

For detailed setup instructions, see the **[User Guide](./docs/guides/USER_GUIDE.md)**.

## 🎯 Core Capabilities

### Real-Time Network Analysis
- **Live packet capture** with sub-second latency
- **Protocol identification** (HTTP, HTTPS, DNS, SSH, etc.)
- **Bandwidth monitoring** with incoming/outgoing separation
- **Host discovery** with reverse DNS lookup
- **Geographic mapping** of network connections

### Advanced Traffic Intelligence
- **Packet deduplication** prevents double-counting
- **Service detection** identifies running network services
- **Connection tracking** monitors active network sessions
- **Performance metrics** with detailed analytics

### Data Management
- **Encrypted storage** with Ring cryptography
- **Atomic operations** ensure data consistency
- **Automatic backups** with configurable retention
- **Session persistence** across application restarts

## 🔧 Development

### Project Structure
```
├── src/                    # React frontend
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── traffic_monitor/   # Network monitoring modules
│   │   ├── commands.rs        # Tauri command handlers
│   │   └── main.rs           # Application entry point
├── docs/                  # Documentation
│   ├── architecture/      # System design
│   ├── development/       # Dev documentation
│   └── guides/           # User guides
└── .github/workflows/    # CI/CD pipelines
```

### Contributing
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Support

- **Documentation**: [Complete documentation](./docs/)
- **Issues**: [GitHub Issues](https://github.com/omnimuh730/rs-fairsight/issues)
- **Discussions**: [GitHub Discussions](https://github.com/omnimuh730/rs-fairsight/discussions)

---

**Made with ❤️ using Rust and React** | **Network monitoring made simple and powerful**

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/omnimuh730/rs-fairsight.git
cd rs-fairsight
```

2. Install dependencies:
```bash
npm install
```

3. Run in development mode:
```bash
npm run tauri dev
```

4. Build for production:
```bash
npm run tauri build
```

## 📊 Key Features by Version

- **v0.1.0**: Foundation with basic time tracking and weekly reports
- **v0.2.0**: Cross-platform support and data encryption
- **v0.2.5**: Server integration and stability improvements
- **v0.5.0**: Comprehensive backup and recovery system
- **v0.9.0**: Advanced health monitoring and modular architecture
- **v1.0.0**: Network monitoring, packet capture, and production release

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
