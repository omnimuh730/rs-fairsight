import React from 'react';
import { Grid, Card, CardContent, Typography, Box } from '@mui/material';
import { Router, CheckCircle, Speed, Storage } from '@mui/icons-material';
import { formatBytes } from '../utils/formatters';

const StatsCards = ({
	totalAdapters,
	activeAdapters,
	monitoringCount,
	totalStats,
	lifetimeStats,
	showLifetime = false
}) => {
	const currentData = showLifetime && lifetimeStats ? 
		{
			totalIncoming: lifetimeStats.lifetime_incoming_bytes || 0,
			totalOutgoing: lifetimeStats.lifetime_outgoing_bytes || 0
		} :
		{
			totalIncoming: totalStats?.totalIncoming || 0,
			totalOutgoing: totalStats?.totalOutgoing || 0
		};

	const formatBytes = (bytes) => {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};
	return (
		<Box sx={{ flexGrow: 1 }}>
			<Grid container spacing={2}>
				<Grid size={{ md: 6, lg: 3 }}>
					<Card sx={{ height: '100%' }}>
						<CardContent sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100%' }}>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Router color="primary" sx={{ mr: 1 }} />
								<Typography variant="h6">Total Adapters</Typography>
							</Box>
							<Typography variant="h3" color="primary">
								{totalAdapters}
							</Typography>
						</CardContent>
					</Card>
				</Grid>

				<Grid size={{ md: 6, lg: 3 }}>
					<Card sx={{ height: '100%' }}>
						<CardContent sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100%' }}>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<CheckCircle color="success" sx={{ mr: 1 }} />
								<Typography variant="h6">Active</Typography>
							</Box>
							<Typography variant="h3" color="success.main">
								{activeAdapters}
							</Typography>
						</CardContent>
					</Card>
				</Grid>

				<Grid size={{ md: 6, lg: 3 }}>
					<Card sx={{ height: '100%' }}>
						<CardContent sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100%' }}>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Speed color="info" sx={{ mr: 1 }} />
								<Typography variant="h6">Monitoring</Typography>
							</Box>
							<Typography variant="h3" color="info.main">
								{monitoringCount}
							</Typography>
							<Typography variant="caption" color="text.secondary">
								Active Sessions
							</Typography>
						</CardContent>
					</Card>
				</Grid>

				<Grid size={{ md: 6, lg: 3 }}>
					<Card sx={{ height: '100%' }}>
						<CardContent sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100%' }}>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Storage color="warning" sx={{ mr: 1 }} />
								<Typography variant="h6">
									{showLifetime ? 'Lifetime Data' : 'Session Data'}
								</Typography>
							</Box>
							<Typography variant="h3" color="warning.main">
								{formatBytes(currentData.totalIncoming + currentData.totalOutgoing)}
							</Typography>
							<Typography variant="caption" color="text.secondary">
								↓ {formatBytes(currentData.totalIncoming)} ↑ {formatBytes(currentData.totalOutgoing)}
							</Typography>
						</CardContent>
					</Card>
				</Grid>
			</Grid>
		</Box>
	);
};

export default StatsCards;
