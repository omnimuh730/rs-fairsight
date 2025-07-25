import React from 'react';
import { Card, CardContent, Typography, Box } from '@mui/material';
import {
	BarChart,
	Bar,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	Legend,
	ResponsiveContainer
} from 'recharts';

/**
 * Hosts and Services Bar Chart Component
 */
const HostsServicesChart = ({ chartData }) => {
	return (
		<Card sx={{ 
			height: { xs: 400, md: 450 }, 
			boxShadow: '0 4px 20px rgba(0,0,0,0.08)',
			borderRadius: 2,
			border: '1px solid rgba(0,0,0,0.08)',
			display: 'flex',
			flexDirection: 'column',
			m: 0
		}}>
			<CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column' }}>
				<Typography variant="h6" gutterBottom sx={{ fontWeight: 600, mb: 2, fontSize: { xs: '1rem', md: '1.25rem' } }}>
					Daily Hosts & Services
				</Typography>
				<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
					<ResponsiveContainer width="100%" height="100%">
						<BarChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 20 }}>
							<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
							<XAxis 
								dataKey="date" 
								fontSize={12}
								tick={{ fill: '#666' }}
								axisLine={{ stroke: '#ddd' }}
							/>
							<YAxis 
								fontSize={12}
								tick={{ fill: '#666' }}
								axisLine={{ stroke: '#ddd' }}
							/>
							<Tooltip 
								contentStyle={{
									backgroundColor: '#fff',
									border: '1px solid #ddd',
									borderRadius: 8,
									boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
								}}
							/>
							<Legend />
							<Bar dataKey="hosts" fill="#2196f3" name="Unique Hosts" radius={[4, 4, 0, 0]} />
							<Bar dataKey="services" fill="#ff9800" name="Unique Services" radius={[4, 4, 0, 0]} />
						</BarChart>
					</ResponsiveContainer>
				</Box>
			</CardContent>
		</Card>
	);
};

export default HostsServicesChart;
