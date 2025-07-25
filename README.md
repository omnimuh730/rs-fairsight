# RS-FairSight

A powerful cross-platform time tracking and monitoring application built with Tauri (Rust) and React. RS-FairSight provides comprehensive activity monitoring, health tracking, and data management with a beautiful, intuitive interface.

## 🚀 Features

- **Real-time Activity Monitoring**: Track your computer usage and productivity patterns
- **Network Traffic Monitoring**: Advanced network packet capture and traffic analysis with real-time statistics
- **Weekly Network Analytics**: Comprehensive network activity reports with detailed charts and insights
- **Health Monitoring System**: Get intelligent warnings and insights about your work habits
- **Cross-Platform Support**: Works seamlessly on Windows, macOS, and Linux
- **Secure Data Storage**: Encrypted local database for your privacy
- **Backup & Recovery**: Robust data backup system with atomic operations and daily cleanup
- **Server Synchronization**: Sync your data across devices
- **Beautiful Analytics**: Comprehensive charts and weekly reports for both activity and network data
- **System Tray Integration**: Runs quietly in the background with real-time monitoring
- **Auto-Startup**: Automatically launches with your system

## 📋 Version History

For detailed information about the evolution of RS-FairSight, see [VERSION_EVOLUTION.md](./VERSION_EVOLUTION.md).

**Current Version**: v1.0.0 (July 2025)

## 🛠 Technology Stack

- **Backend**: Rust with Tauri framework
- **Frontend**: React with Material-UI (MUI)
- **Build Tool**: Vite
- **Charts**: MUI X-Charts
- **Date Management**: Day.js and date-fns
- **Routing**: React Router DOM

## 🏗 Architecture

The application features a modular architecture with separated concerns:

```
src-tauri/src/
├── app_state.rs         # Application state management
├── commands.rs          # Tauri command handlers  
├── encryption.rs        # Data encryption utilities
├── file_utils.rs        # File system operations
├── health_monitor.rs    # Activity monitoring
├── logger.rs            # Logging infrastructure
├── network_monitor.rs   # Network traffic monitoring
├── real_traffic_monitor.rs # Real-time packet capture
├── time_tracker.rs      # Core time tracking logic
├── traffic_monitor.rs   # Traffic analysis and statistics
└── web_server.rs        # Server communication
```

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
