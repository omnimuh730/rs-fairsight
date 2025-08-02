import { 
	Rocket as RocketIcon,
	Security as SecurityIcon,
	Storage as StorageIcon,
	Monitor as MonitorIcon,
	Build as BuildIcon,
	IntegrationInstructions as IntegrationIcon,
	Description as DocumentationIcon
} from '@mui/icons-material';

export const versionData = [
	{
		version: 'v1.1.0',
		date: 'August 1, 2025',
		title: 'CI/CD & Documentation Organization',
		description: 'Complete infrastructure overhaul with automated CI/CD pipeline and comprehensive documentation restructure',
		icon: <IntegrationIcon />,
		color: 'info',
		features: [
			'GitHub Actions CI/CD Pipeline',
			'Cross-platform Automated Builds',
			'Comprehensive Documentation Restructure',
			'Version Tagging System',
			'Dependency Resolution Improvements',
			'Real Data Only (No Simulation)',
			'Traffic Monitor Modular Refactoring',
			'Enhanced User Feedback Systems'
		],
		commits: 'ba748582',
		highlights: [
			'🏗️ Complete CI/CD infrastructure',
			'📚 Documentation organization',
			'🔧 Development workflow improvements',
			'🌐 Data authenticity guarantees'
		]
	},
	{
		version: 'v1.0.0',
		date: 'July 25, 2025',
		title: 'Network Monitoring & Production Release',
		description: 'Milestone release with comprehensive network monitoring and production-ready features',
		icon: <MonitorIcon />,
		color: 'success',
		features: [
			'Advanced Network Traffic Monitoring',
			'Real-time Packet Capture Engine',
			'Weekly Network Activity Analytics',
			'Network Data Backup System',
			'Enhanced UI with Country Flags',
			'Performance Optimizations',
			'Production-Ready Architecture',
			'Security & Privacy Enhancements'
		],
		commits: '86d772ce',
		highlights: [
			'🌐 Production-ready network monitoring',
			'📊 Advanced analytics engine',
			'🔒 Enhanced security features',
			'⚡ Performance optimizations'
		]
	},
	{
		version: 'v0.9.0',
		date: 'July 21, 2025',
		title: 'Advanced Monitoring & Modular Architecture',
		description: 'Complete architectural overhaul with advanced monitoring capabilities',
		icon: <MonitorIcon />,
		color: 'primary',
		features: [
			'Modular Architecture Refactor',
			'Advanced Health Monitoring System',
			'Comprehensive Logging Infrastructure',
			'Atomic Backup Operations',
			'Enhanced Error Handling',
			'Intelligent Traffic Analysis',
			'Host Analysis & Geolocation',
			'Service Detection Capabilities'
		],
		commits: 'd3a20af4',
		highlights: [
			'🏗️ Complete architecture overhaul',
			'🧠 Intelligent monitoring systems',
			'📋 Comprehensive logging',
			'🔄 Atomic operations'
		]
	},
	{
		version: 'v0.5.0',
		date: 'May 22, 2025',
		title: 'Real-time Network Monitoring Foundation',
		description: 'Major infrastructure upgrade with real-time network monitoring and capture capabilities',
		icon: <StorageIcon />,
		color: 'secondary',
		features: [
			'Real-time Packet Capture',
			'Network Adapter Discovery',
			'Basic Traffic Analysis',
			'Host Identification System',
			'Async Processing Framework',
			'Concurrent Data Structures',
			'Cross-platform Compatibility',
			'Event-driven Architecture'
		],
		commits: 'e35e7491',
		highlights: [
			'🌐 Real-time network monitoring',
			'🔄 Async processing foundation',
			'💾 Advanced data structures',
			'🖥️ Cross-platform support'
		]
	},
	{
		version: 'v0.2.5',
		date: 'April 12, 2025',
		title: 'Server Integration & Stability',
		description: 'Server-side synchronization and application stability improvements',
		icon: <BuildIcon />,
		color: 'warning',
		features: [
			'Server Endpoint Integration',
			'macOS Dock Icon Fix',
			'Changelog System',
			'CORS Error Resolution',
			'Version Management',
			'Basic Activity Monitoring',
			'Data Persistence Layer',
			'System Tray Integration'
		],
		commits: '1f699f54',
		highlights: [
			'🔗 Server integration',
			'🍎 macOS optimizations',
			'📋 Changelog system',
			'🔧 Stability improvements'
		]
	},
	{
		version: 'v0.2.0',
		date: 'April 4, 2025',
		title: 'Cross-Platform Expansion',
		description: 'Major expansion with cross-platform compatibility and advanced features',
		icon: <SecurityIcon />,
		color: 'secondary',
		features: [
			'Cross-Platform macOS Support',
			'Auto-Startup Functionality',
			'Data Encryption & Security',
			'Advanced Logging System',
			'UI Theme Enhancements',
			'Performance Optimizations',
			'Error Handling Improvements',
			'Development Tools'
		],
		commits: 'a5e1273',
		highlights: [
			'🖥️ Cross-platform support',
			'🔒 Data encryption',
			'🎨 UI enhancements',
			'🚀 Auto-startup features'
		]
	},
	{
		version: 'v0.1.0',
		date: 'March 3, 2025',
		title: 'Foundation Release',
		description: 'Initial foundation with core infrastructure and basic functionality',
		icon: <RocketIcon />,
		color: 'info',
		features: [
			'Basic Time Tracking',
			'Chart Visualization',
			'Weekly Reporting',
			'System Tray Integration',
			'Performance Optimization',
			'Tauri + React Architecture',
			'Basic UI Layout',
			'Foundation Infrastructure'
		],
		commits: '2802288',
		highlights: [
			'🚀 Project foundation',
			'📊 Chart visualization',
			'⏰ Time tracking basics',
			'🎯 System integration'
		]
	}
];
