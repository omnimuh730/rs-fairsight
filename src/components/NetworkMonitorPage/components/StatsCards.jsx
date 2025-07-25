import React from 'react';
import { Grid, Card, CardContent, Typography, Box } from '@mui/material';
import { Router, CheckCircle, Speed, Storage } from '@mui/icons-material';
import { formatBytes } from '../utils/formatters';

const StatsCards = ({ 
	totalAdapters, 
	activeAdapters, 
	monitoringCount, 
	totalStats 
}) => {
	return (
		<Grid container spacing={2}>
			<Grid item xs={12} sm={6} md={3}>
				<Card>
					<CardContent>
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
			
			<Grid item xs={12} sm={6} md={3}>
				<Card>
					<CardContent>
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
			
			<Grid item xs={12} sm={6} md={3}>
				<Card>
					<CardContent>
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
			
			<Grid item xs={12} sm={6} md={3}>
				<Card>
					<CardContent>
						<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
							<Storage color="warning" sx={{ mr: 1 }} />
							<Typography variant="h6">Total Data</Typography>
						</Box>
						<Typography variant="h3" color="warning.main">
							{formatBytes(totalStats.totalIncoming + totalStats.totalOutgoing)}
						</Typography>
						<Typography variant="caption" color="text.secondary">
							↓ {formatBytes(totalStats.totalIncoming)} ↑ {formatBytes(totalStats.totalOutgoing)}
						</Typography>
					</CardContent>
				</Card>
			</Grid>
		</Grid>
	);
};

export default StatsCards;
