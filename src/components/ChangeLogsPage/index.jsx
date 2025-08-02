import * as React from 'react';
import { Typography, Box } from '@mui/material';
import { versionData } from './versionData.jsx';
import VersionTimeline from './VersionTimeline.jsx';
import { createAnimationStyles } from './animations';
import { AnimatedTitle } from './StyledComponents.jsx';

export default function ChangeLogsPage() {
	// Add CSS animations
	React.useEffect(() => {
		const style = createAnimationStyles();
		document.head.appendChild(style);
		
		return () => {
			if (document.head.contains(style)) {
				document.head.removeChild(style);
			}
		};
	}, []);

	return (
		<Box sx={{ padding: 3 }}>
			<AnimatedTitle variant="h3" component="h1" gutterBottom>
				📋 Version Changelog
			</AnimatedTitle>
			
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
				Track the evolution of InnoMonitor through its major releases
			</Typography>

			<VersionTimeline versionData={versionData} />

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
