import React from 'react';
import { Card, CardContent, Typography, Box } from '@mui/material';
import {
	PieChart,
	Pie,
	Cell,
	Tooltip,
	ResponsiveContainer
} from 'recharts';
import { formatBytes } from '../utils/formatters';

/**
 * Traffic Distribution Pie Chart Component
 */
const TrafficDistributionChart = ({ pieData }) => {
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
					Traffic Distribution
				</Typography>
				<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
					<ResponsiveContainer width="100%" height="100%">
						<PieChart>
							<Pie
								data={pieData}
								cx="50%"
								cy="50%"
								labelLine={false}
								label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
								outerRadius={120}
								fill="#8884d8"
								dataKey="value"
							>
								{pieData.map((entry, index) => (
									<Cell key={`cell-${index}`} fill={entry.color} />
								))}
							</Pie>
							<Tooltip 
								formatter={(value) => formatBytes(value)}
								contentStyle={{
									backgroundColor: '#fff',
									border: '1px solid #ddd',
									borderRadius: 8,
									boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
								}}
							/>
						</PieChart>
					</ResponsiveContainer>
				</Box>
			</CardContent>
		</Card>
	);
};

export default TrafficDistributionChart;
