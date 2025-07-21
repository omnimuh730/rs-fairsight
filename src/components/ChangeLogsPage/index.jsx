import * as React from 'react';
import Timeline from '@mui/lab/Timeline';
import TimelineItem from '@mui/lab/TimelineItem';
import TimelineSeparator from '@mui/lab/TimelineSeparator';
import TimelineConnector from '@mui/lab/TimelineConnector';
import TimelineContent from '@mui/lab/TimelineContent';
import TimelineDot from '@mui/lab/TimelineDot';
import TimelineOppositeContent from '@mui/lab/TimelineOppositeContent';
import { Typography, Box, Chip, Card, CardContent } from '@mui/material';
import { 
	Rocket as RocketIcon,
	Security as SecurityIcon,
	Storage as StorageIcon,
	Monitor as MonitorIcon,
	Build as BuildIcon
} from '@mui/icons-material';

const versionData = [
	{
		version: 'v0.9.0',
		date: 'July 20-21, 2025',
		title: 'Advanced Monitoring & Modular Architecture',
		description: 'Complete architectural overhaul with advanced monitoring capabilities',
		icon: <MonitorIcon />,
		color: 'primary',
		features: [
			'Modular Architecture Refactor',
			'Advanced Health Monitoring System',
			'Comprehensive Logging Infrastructure',
			'Atomic Backup Operations',
			'Enhanced Error Handling'
		],
		commits: 'e35e749 → 3ff7ba5'
	},
	{
		version: 'v0.5.0',
		date: 'May 18-22, 2025',
		title: 'Backup & Recovery System',
		description: 'Major infrastructure upgrade with backup and recovery capabilities',
		icon: <StorageIcon />,
		color: 'secondary',
		features: [
			'Data Backup & Restore',
			'Backup Validation System',
			'macOS Backup Integration',
			'Dependency Optimization',
			'Data Integrity Checks'
		],
		commits: '08ad4ee → 19fff96'
	},
	{
		version: 'v0.2.5',
		date: 'April 4-12, 2025',
		title: 'Server Integration & Stability',
		description: 'Server-side synchronization and application stability improvements',
		icon: <BuildIcon />,
		color: 'success',
		features: [
			'Server Endpoint Integration',
			'macOS Dock Icon Fix',
			'Changelog System',
			'CORS Error Resolution',
			'Version Management'
		],
		commits: '897a49d → 1f699f5'
	},
	{
		version: 'v0.2.0',
		date: 'March 15 - April 4, 2025',
		title: 'Cross-Platform Expansion',
		description: 'Major expansion with cross-platform compatibility and advanced features',
		icon: <SecurityIcon />,
		color: 'warning',
		features: [
			'Cross-Platform macOS Support',
			'Auto-Startup Functionality',
			'Data Encryption & Security',
			'Advanced Logging System',
			'UI Theme Enhancements'
		],
		commits: '01f440a → a5e1273'
	},
	{
		version: 'v0.1.0',
		date: 'March 3-9, 2025',
		title: 'Foundation Release',
		description: 'Initial foundation with core infrastructure and basic functionality',
		icon: <RocketIcon />,
		color: 'info',
		features: [
			'Basic Time Tracking',
			'Chart Visualization',
			'Weekly Reporting',
			'System Tray Integration',
			'Performance Optimization'
		],
		commits: '224ebb6 → 2802288'
	}
];

export default function ChangeLogsPage() {
	return (
		<Box sx={{ padding: 3 }}>
			<Typography variant="h3" component="h1" gutterBottom sx={{ 
				textAlign: 'center', 
				fontWeight: 'bold',
				background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
				WebkitBackgroundClip: 'text',
				WebkitTextFillColor: 'transparent',
				marginBottom: 4
			}}>
				📋 Version Changelog
			</Typography>
			
			<Typography variant="h6" sx={{ 
				textAlign: 'center', 
				color: 'text.secondary', 
				marginBottom: 4 
			}}>
				Track the evolution of RS-FairSight through its major releases
			</Typography>

			<Timeline position="alternate">
				{versionData.map((version, index) => (
					<TimelineItem key={version.version}>
						<TimelineOppositeContent
							sx={{ m: 'auto 0' }}
							align={index % 2 === 0 ? "right" : "left"}
							variant="body2"
							color="text.secondary"
						>
							<Typography variant="subtitle2" fontWeight="bold">
								{version.date}
							</Typography>
							<Typography variant="caption" sx={{ display: 'block', mt: 0.5 }}>
								Commits: {version.commits}
							</Typography>
						</TimelineOppositeContent>
						
						<TimelineSeparator>
							<TimelineDot color={version.color} variant="outlined">
								{version.icon}
							</TimelineDot>
							{index < versionData.length - 1 && <TimelineConnector />}
						</TimelineSeparator>
						
						<TimelineContent sx={{ py: '12px', px: 2 }}>
							<Card elevation={3} sx={{ 
								maxWidth: 400,
								transition: 'transform 0.2s, box-shadow 0.2s',
								'&:hover': {
									transform: 'translateY(-2px)',
									boxShadow: 6
								}
							}}>
								<CardContent>
									<Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
										<Chip 
											label={version.version} 
											color={version.color}
											variant="filled"
											sx={{ fontWeight: 'bold', fontSize: '0.9rem' }}
										/>
									</Box>
									
									<Typography variant="h6" component="h3" gutterBottom fontWeight="bold">
										{version.title}
									</Typography>
									
									<Typography variant="body2" color="text.secondary" paragraph>
										{version.description}
									</Typography>
									
									<Typography variant="subtitle2" fontWeight="bold" gutterBottom>
										Key Features:
									</Typography>
									
									<Box component="ul" sx={{ 
										margin: 0, 
										paddingLeft: 2,
										'& li': {
											fontSize: '0.875rem',
											marginBottom: 0.5,
											color: 'text.secondary'
										}
									}}>
										{version.features.map((feature, idx) => (
											<li key={idx}>{feature}</li>
										))}
									</Box>
								</CardContent>
							</Card>
						</TimelineContent>
					</TimelineItem>
				))}
			</Timeline>

			<Box sx={{ textAlign: 'center', mt: 4, p: 3, bgcolor: 'background.paper', borderRadius: 2 }}>
				<Typography variant="h6" gutterBottom>
					🚀 What's Next?
				</Typography>
				<Typography variant="body1" color="text.secondary">
					Stay tuned for upcoming features including enhanced analytics, 
					improved cross-platform compatibility, and advanced synchronization capabilities.
				</Typography>
			</Box>
		</Box>
	);
}
