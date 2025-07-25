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
			'Performance Optimizations'
		],
		commits: '0d3b850 → 0755c91'
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
			'Enhanced Error Handling'
		],
		commits: 'e35e749 → 3ff7ba5'
	},
	{
		version: 'v0.5.0',
		date: 'May 22, 2025',
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
		date: 'April 12, 2025',
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
		date: 'April 4, 2025',
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
			'Performance Optimization'
		],
		commits: '224ebb6 → 2802288'
	}
];

export default function ChangeLogsPage() {
	// Add CSS animations
	React.useEffect(() => {
		const style = document.createElement('style');
		style.textContent = `
			@keyframes slideInFromSide {
				from {
					opacity: 0;
					transform: translateX(-30px);
				}
				to {
					opacity: 1;
					transform: translateX(0);
				}
			}
			
			@keyframes slideInFromContent {
				from {
					opacity: 0;
					transform: translateX(30px) scale(0.9);
				}
				to {
					opacity: 1;
					transform: translateX(0) scale(1);
				}
			}
			
			@keyframes bounceIn {
				0% {
					opacity: 0;
					transform: scale(0.3) rotate(-180deg);
				}
				50% {
					opacity: 0.8;
					transform: scale(1.1) rotate(-90deg);
				}
				100% {
					opacity: 1;
					transform: scale(1) rotate(0deg);
				}
			}
			
			@keyframes slideDown {
				from {
					opacity: 0;
					height: 0;
				}
				to {
					opacity: 1;
					height: 100%;
				}
			}
			
			@keyframes pulse {
				0%, 100% {
					opacity: 0.3;
				}
				50% {
					opacity: 0.8;
				}
			}
			
			@keyframes spin {
				from {
					transform: rotate(0deg) scale(1.2);
				}
				to {
					transform: rotate(360deg) scale(1.2);
				}
			}
			
			@keyframes wiggle {
				0%, 100% {
					transform: rotate(0deg);
				}
				25% {
					transform: rotate(-3deg) scale(1.05);
				}
				75% {
					transform: rotate(3deg) scale(1.05);
				}
			}
			
			@keyframes sparkle {
				0% {
					opacity: 0;
					transform: scale(0) rotate(0deg);
				}
				50% {
					opacity: 1;
					transform: scale(1.2) rotate(180deg);
				}
				100% {
					opacity: 0;
					transform: scale(0) rotate(360deg);
				}
			}
			
			@keyframes fadeInUp {
				from {
					opacity: 0;
					transform: translateY(30px);
				}
				to {
					opacity: 1;
					transform: translateY(0);
				}
			}
			
			@keyframes gradientShift {
				0% {
					background-position: 0% 50%;
				}
				50% {
					background-position: 100% 50%;
				}
				100% {
					background-position: 0% 50%;
				}
			}
		`;
		document.head.appendChild(style);
		
		return () => {
			document.head.removeChild(style);
		};
	}, []);

	return (
		<Box sx={{ padding: 3 }}>
			<Typography variant="h3" component="h1" gutterBottom sx={{ 
				textAlign: 'center', 
				fontWeight: 'bold',
				background: 'linear-gradient(-45deg, #2196F3, #21CBF3, #9C27B0, #E1BEE7)',
				backgroundSize: '400% 400%',
				WebkitBackgroundClip: 'text',
				WebkitTextFillColor: 'transparent',
				marginBottom: 4,
				animation: 'gradientShift 4s ease infinite, fadeInUp 1s ease-out',
				fontSize: { xs: '2rem', md: '3rem' }
			}}>
				📋 Version Changelog
			</Typography>
			
			<Typography variant="h6" sx={{ 
				textAlign: 'center', 
				color: 'text.secondary', 
				marginBottom: 4,
				animation: 'fadeInUp 1s ease-out 0.3s both',
				'&:hover': {
					color: 'primary.main',
					transition: 'color 0.3s ease'
				}
			}}>
				Track the evolution of RS-FairSight through its major releases
			</Typography>

			<Timeline position="alternate">
				{versionData.map((version, index) => (
					<TimelineItem 
						key={version.version}
						sx={{
							'&::before': {
								content: '""',
								position: 'absolute',
								top: 0,
								left: index % 2 === 0 ? 'auto' : 0,
								right: index % 2 === 0 ? 0 : 'auto',
								width: '2px',
								height: '100%',
								background: `linear-gradient(180deg, transparent, ${
									version.color === 'primary' ? '#2196F3' :
									version.color === 'secondary' ? '#9C27B0' :
									version.color === 'success' ? '#4CAF50' :
									version.color === 'warning' ? '#FF9800' :
									'#2196F3'
								}, transparent)`,
								opacity: 0.3,
								animation: 'pulse 2s infinite'
							}
						}}
					>
						<TimelineOppositeContent
							sx={{ 
								m: 'auto 0',
								textAlign: index % 2 === 0 ? 'right' : 'left',
								animation: 'slideInFromSide 0.8s ease-out',
								animationDelay: `${index * 0.2}s`,
								animationFillMode: 'both'
							}}
							variant="body2"
							color="text.secondary"
						>
							<Typography 
								variant="subtitle2" 
								fontWeight="bold"
								sx={{
									background: `linear-gradient(45deg, ${
										version.color === 'primary' ? '#2196F3, #21CBF3' :
										version.color === 'secondary' ? '#9C27B0, #E1BEE7' :
										version.color === 'success' ? '#4CAF50, #81C784' :
										version.color === 'warning' ? '#FF9800, #FFB74D' :
										'#2196F3, #21CBF3'
									})`,
									WebkitBackgroundClip: 'text',
									WebkitTextFillColor: 'transparent',
									fontSize: '1rem'
								}}
							>
								{version.date}
							</Typography>
							<Typography 
								variant="caption" 
								sx={{ 
									display: 'block', 
									mt: 0.5,
									opacity: 0.8,
									fontFamily: 'monospace'
								}}
							>
								Commits: {version.commits}
							</Typography>
						</TimelineOppositeContent>
						
						<TimelineSeparator>
							<TimelineDot 
								color={version.color} 
								variant="outlined"
								sx={{
									animation: 'bounceIn 1s ease-out',
									animationDelay: `${index * 0.2 + 0.3}s`,
									animationFillMode: 'both',
									'&:hover': {
										animation: 'spin 1s ease-in-out',
										transform: 'scale(1.2)'
									},
									transition: 'all 0.3s ease'
								}}
							>
								{version.icon}
							</TimelineDot>
							{index < versionData.length - 1 && (
								<TimelineConnector 
									sx={{
										background: `linear-gradient(180deg, ${
											version.color === 'primary' ? '#2196F3' :
											version.color === 'secondary' ? '#9C27B0' :
											version.color === 'success' ? '#4CAF50' :
											version.color === 'warning' ? '#FF9800' :
											'#2196F3'
										}, transparent)`,
										opacity: 0.6,
										animation: 'slideDown 1s ease-out',
										animationDelay: `${index * 0.2 + 0.5}s`,
										animationFillMode: 'both'
									}}
								/>
							)}
						</TimelineSeparator>
						
						<TimelineContent sx={{ py: '12px', px: 2 }}>
							<Card 
								elevation={3} 
								sx={{ 
									maxWidth: 400,
									transition: 'all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275)',
									animation: 'slideInFromContent 0.8s ease-out',
									animationDelay: `${index * 0.2 + 0.1}s`,
									animationFillMode: 'both',
									background: 'linear-gradient(145deg, rgba(255,255,255,0.1), rgba(255,255,255,0.05))',
									backdropFilter: 'blur(10px)',
									border: '1px solid rgba(255,255,255,0.2)',
									'&:hover': {
										transform: 'translateY(-8px) scale(1.02)',
										boxShadow: `0 20px 40px rgba(${
											version.color === 'primary' ? '33, 150, 243' :
											version.color === 'secondary' ? '156, 39, 176' :
											version.color === 'success' ? '76, 175, 80' :
											version.color === 'warning' ? '255, 152, 0' :
											'33, 150, 243'
										}, 0.3)`,
										'& .MuiChip-root': {
											animation: 'wiggle 0.5s ease-in-out'
										}
									}
								}}
							>
								<CardContent>
									<Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
										<Chip 
											label={version.version} 
											color={version.color}
											variant="filled"
											sx={{ 
												fontWeight: 'bold', 
												fontSize: '0.9rem',
												background: `linear-gradient(45deg, ${
													version.color === 'primary' ? '#2196F3, #21CBF3' :
													version.color === 'secondary' ? '#9C27B0, #E1BEE7' :
													version.color === 'success' ? '#4CAF50, #81C784' :
													version.color === 'warning' ? '#FF9800, #FFB74D' :
													'#2196F3, #21CBF3'
												})`,
												boxShadow: '0 4px 15px rgba(0,0,0,0.2)',
												transition: 'all 0.3s ease'
											}}
										/>
									</Box>
									
									<Typography 
										variant="h6" 
										component="h3" 
										gutterBottom 
										fontWeight="bold"
										sx={{
											background: 'linear-gradient(45deg, #333, #666)',
											WebkitBackgroundClip: 'text',
											WebkitTextFillColor: 'transparent'
										}}
									>
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
											color: 'text.secondary',
											position: 'relative',
											transition: 'all 0.3s ease',
											'&:hover': {
												color: 'text.primary',
												transform: 'translateX(5px)',
												'&::before': {
													content: '"✨"',
													position: 'absolute',
													left: '-20px',
													animation: 'sparkle 0.6s ease-out'
												}
											}
										}
									}}>
										{version.features.map((feature, idx) => (
											<li 
												key={idx}
												style={{
													animationDelay: `${index * 0.2 + idx * 0.1 + 0.8}s`
												}}
											>
												{feature}
											</li>
										))}
									</Box>
								</CardContent>
							</Card>
						</TimelineContent>
					</TimelineItem>
				))}
			</Timeline>

			<Box sx={{ 
				textAlign: 'center', 
				mt: 4, 
				p: 3, 
				bgcolor: 'background.paper', 
				borderRadius: 3,
				background: 'linear-gradient(145deg, rgba(255,255,255,0.1), rgba(255,255,255,0.05))',
				backdropFilter: 'blur(10px)',
				border: '1px solid rgba(255,255,255,0.2)',
				animation: 'fadeInUp 1s ease-out 2s both',
				transition: 'all 0.3s ease',
				'&:hover': {
					transform: 'translateY(-5px)',
					boxShadow: '0 15px 35px rgba(33, 150, 243, 0.2)',
					'& h6': {
						animation: 'wiggle 0.5s ease-in-out'
					}
				}
			}}>
				<Typography 
					variant="h6" 
					gutterBottom
					sx={{
						background: 'linear-gradient(45deg, #2196F3, #21CBF3)',
						WebkitBackgroundClip: 'text',
						WebkitTextFillColor: 'transparent',
						fontWeight: 'bold'
					}}
				>
					🚀 What's Next?
				</Typography>
				<Typography 
					variant="body1" 
					color="text.secondary"
					sx={{
						'&:hover': {
							color: 'text.primary',
							transition: 'color 0.3s ease'
						}
					}}
				>
					Stay tuned for upcoming features including enhanced analytics, 
					improved cross-platform compatibility, and advanced synchronization capabilities.
				</Typography>
			</Box>
		</Box>
	);
}
