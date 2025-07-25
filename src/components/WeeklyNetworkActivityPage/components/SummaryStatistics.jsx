import React from 'react';
import { Card, CardContent, Typography, Box, Grid } from '@mui/material';
import {
	TrendingUp,
	CloudDownload,
	CloudUpload,
	Schedule,
	Computer,
	NetworkCheck
} from '@mui/icons-material';
import StatCard from './StatCard';
import { formatBytes, formatDuration } from '../utils/formatters';

/**
 * Summary statistics section component
 */
const SummaryStatistics = ({ totalStats }) => {
	return (
		<Card sx={{ 
			mb: 4,
			boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
			borderRadius: 3,
			border: '1px solid rgba(255,255,255,0.2)'
		}}>
			<CardContent sx={{ p: { xs: 3, md: 4 } }}>
				<Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
					<TrendingUp sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
					<Typography variant="h5" sx={{ fontWeight: 600, color: 'text.primary' }}>
						Summary Statistics
					</Typography>
				</Box>
				<Grid container spacing={3}>
					<Grid item xs={6} sm={4} md={2}>
						<StatCard
							icon={CloudDownload}
							title="Incoming"
							value={formatBytes(totalStats.totalIncoming)}
							color="primary"
							colorValue="primary.main"
						/>
					</Grid>
					<Grid item xs={6} sm={4} md={2}>
						<StatCard
							icon={CloudUpload}
							title="Outgoing"
							value={formatBytes(totalStats.totalOutgoing)}
							color="secondary"
							colorValue="secondary.main"
						/>
					</Grid>
					<Grid item xs={6} sm={4} md={2}>
						<StatCard
							icon={Schedule}
							title="Duration"
							value={formatDuration(totalStats.totalDuration)}
							color="info"
							colorValue="info.main"
						/>
					</Grid>
					<Grid item xs={6} sm={4} md={2}>
						<StatCard
							icon={Computer}
							title="Hosts"
							value={totalStats.uniqueHosts}
							color="success"
							colorValue="success.main"
						/>
					</Grid>
					<Grid item xs={6} sm={4} md={2}>
						<StatCard
							icon={TrendingUp}
							title="Services"
							value={totalStats.uniqueServices}
							color="warning"
							colorValue="warning.main"
						/>
					</Grid>
					<Grid item xs={6} sm={4} md={2}>
						<StatCard
							icon={NetworkCheck}
							title="Sessions"
							value={totalStats.totalSessions}
							color="purple"
							colorValue="#9c27b0"
						/>
					</Grid>
				</Grid>
			</CardContent>
		</Card>
	);
};

export default SummaryStatistics;
