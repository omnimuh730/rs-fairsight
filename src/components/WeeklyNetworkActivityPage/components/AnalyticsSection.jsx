import React from 'react';
import { Card, CardContent, Typography, Box, Grid } from '@mui/material';
import { TrendingUp } from '@mui/icons-material';
import DailyTrafficChart from './DailyTrafficChart';
import TrafficDistributionChart from './TrafficDistributionChart';
import SessionsDurationChart from './SessionsDurationChart';
import HostsServicesChart from './HostsServicesChart';

/**
 * Analytics and Trends section containing all charts
 */
const AnalyticsSection = ({ chartData, pieData }) => {
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
						Analytics & Trends
					</Typography>
				</Box>
				
				{/* First Row - 2 Charts */}
				<Grid container spacing={0} sx={{ mb: 0 }}>
					<Grid item xs={12} md={6} sx={{ pr: { md: 1 } }}>
						<DailyTrafficChart chartData={chartData} />
					</Grid>
					<Grid item xs={12} md={6} sx={{ pl: { md: 1 } }}>
						<TrafficDistributionChart pieData={pieData} />
					</Grid>
				</Grid>

				{/* Second Row - 2 Charts */}
				<Grid container spacing={0} sx={{ mt: 2 }}>
					<Grid item xs={12} md={6} sx={{ pr: { md: 1 } }}>
						<SessionsDurationChart chartData={chartData} />
					</Grid>
					<Grid item xs={12} md={6} sx={{ pl: { md: 1 } }}>
						<HostsServicesChart chartData={chartData} />
					</Grid>
				</Grid>
			</CardContent>
		</Card>
	);
};

export default AnalyticsSection;
