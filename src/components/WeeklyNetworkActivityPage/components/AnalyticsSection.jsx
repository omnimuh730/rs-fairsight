import React from 'react';
import { Card, CardContent, Typography, Box, Grid, Paper } from '@mui/material';
import { TrendingUp } from '@mui/icons-material';
import DailyTrafficChart from './DailyTrafficChart';
import TrafficDistributionChart from './TrafficDistributionChart';
import SessionsDurationChart from './SessionsDurationChart';
import HostsServicesChart from './HostsServicesChart';
import { styled } from '@mui/material/styles';

const Item = styled(Paper)(({ theme }) => ({
	backgroundColor: '#fff',
	...theme.typography.body2,
	padding: theme.spacing(1),
	textAlign: 'center',
	color: (theme.vars ?? theme).palette.text.secondary,
	...theme.applyStyles('dark', {
		backgroundColor: '#1A2027',
	}),
}));


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
				<Box sx={{ flexGrow: 1 }}>
					<Grid container spacing={2}>
						<Grid size={{ md: 12, lg: 6 }}>
							<Item>
								<DailyTrafficChart chartData={chartData} />
							</Item>
						</Grid>
						<Grid size={{ md: 12, lg: 6 }}>
							<Item>
								<TrafficDistributionChart pieData={pieData} />
							</Item>
						</Grid>
						<Grid size={{ md: 12, lg: 6 }}>
							<Item>
								<SessionsDurationChart chartData={chartData} />
							</Item>
						</Grid>

						<Grid size={{ md: 12, lg: 6 }}>
							<Item>
								<HostsServicesChart chartData={chartData} />
							</Item>
						</Grid>
					</Grid>
				</Box>
			</CardContent>
		</Card>
	);
};

export default AnalyticsSection;
