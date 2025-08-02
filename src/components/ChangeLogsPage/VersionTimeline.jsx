import React from 'react';
import { Typography, Box, CardContent } from '@mui/material';
import Timeline from '@mui/lab/Timeline';
import TimelineSeparator from '@mui/lab/TimelineSeparator';
import TimelineConnector from '@mui/lab/TimelineConnector';
import TimelineContent from '@mui/lab/TimelineContent';
import TimelineOppositeContent from '@mui/lab/TimelineOppositeContent';
import { 
	StyledTimelineItem, 
	StyledTimelineDot, 
	VersionCard, 
	GradientChip, 
	HighlightBox,
	OppositeContent 
} from './StyledComponents.jsx';

const VersionTimeline = ({ versionData }) => {
	return (
		<Timeline position="alternate">
			{versionData.map((version, index) => (
				<StyledTimelineItem 
					key={version.version}
					index={index}
				>
					<TimelineOppositeContent>
						<OppositeContent index={index}>
							<Typography 
								variant="subtitle2" 
								fontWeight="bold"
								sx={{
									background: `linear-gradient(45deg, ${
										version.color === 'primary' ? '#2196F3, #21CBF3' :
										version.color === 'secondary' ? '#9C27B0, #E1BEE7' :
										version.color === 'success' ? '#4CAF50, #81C784' :
										version.color === 'warning' ? '#FF9800, #FFB74D' :
										version.color === 'info' ? '#2196F3, #21CBF3' :
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
								Commit: {version.commits}
							</Typography>
						</OppositeContent>
					</TimelineOppositeContent>
					
					<TimelineSeparator>
						<StyledTimelineDot 
							versionColor={version.color}
							variant="outlined"
							sx={{
								animationDelay: `${index * 0.2 + 0.3}s`,
								animationFillMode: 'both',
							}}
						>
							{version.icon}
						</StyledTimelineDot>
						{index < versionData.length - 1 && (
							<TimelineConnector 
								sx={{
									background: `linear-gradient(180deg, ${
										version.color === 'primary' ? '#2196F3' :
										version.color === 'secondary' ? '#9C27B0' :
										version.color === 'success' ? '#4CAF50' :
										version.color === 'warning' ? '#FF9800' :
										version.color === 'info' ? '#2196F3' :
										'#2196F3'
									}, ${
										versionData[index + 1]?.color === 'primary' ? '#2196F3' :
										versionData[index + 1]?.color === 'secondary' ? '#9C27B0' :
										versionData[index + 1]?.color === 'success' ? '#4CAF50' :
										versionData[index + 1]?.color === 'warning' ? '#FF9800' :
										versionData[index + 1]?.color === 'info' ? '#2196F3' :
										'#2196F3'
									})`,
									width: '3px',
									height: '60px',
									opacity: 0.5,
									animation: 'slideDown 1s ease-out',
									animationDelay: `${index * 0.2 + 0.5}s`,
									animationFillMode: 'both',
									margin: 'auto',
									borderRadius: '2px'
								}}
							/>
						)}
					</TimelineSeparator>
					
					<TimelineContent sx={{ py: '12px', px: 2 }}>
						<VersionCard 
							elevation={3}
							versionColor={version.color}
							index={index}
							sx={{ 
								animation: 'slideInFromContent 0.8s ease-out',
								animationDelay: `${index * 0.2 + 0.1}s`,
								animationFillMode: 'both',
							}}
						>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
									<GradientChip 
										label={version.version} 
										versionColor={version.color}
										className="version-chip"
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

								{version.highlights && (
									<HighlightBox>
										<Typography variant="subtitle2" fontWeight="bold" gutterBottom>
											🌟 Key Highlights:
										</Typography>
										{version.highlights.map((highlight, idx) => (
											<Typography key={idx} variant="body2" sx={{ mb: 0.5 }}>
												{highlight}
											</Typography>
										))}
									</HighlightBox>
								)}
								
								<Typography variant="subtitle2" fontWeight="bold" gutterBottom sx={{ mt: 2 }}>
									📋 Features & Improvements:
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
											className="feature-item"
											style={{
												animationDelay: `${index * 0.2 + idx * 0.1 + 0.8}s`
											}}
										>
											{feature}
										</li>
									))}
								</Box>
							</CardContent>
						</VersionCard>
					</TimelineContent>
				</StyledTimelineItem>
			))}
		</Timeline>
	);
};

export default VersionTimeline;
