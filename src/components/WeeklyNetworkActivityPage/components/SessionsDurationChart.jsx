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
 * Sessions and Duration Bar Chart Component
 */
const SessionsDurationChart = ({ chartData }) => {
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
					Daily Sessions & Duration
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
								yAxisId="left" 
								fontSize={12}
								tick={{ fill: '#666' }}
								axisLine={{ stroke: '#ddd' }}
							/>
							<YAxis 
								yAxisId="right" 
								orientation="right" 
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
							<Bar yAxisId="left" dataKey="sessions" fill="#4caf50" name="Sessions" radius={[4, 4, 0, 0]} />
							<Bar yAxisId="right" dataKey="duration" fill="#9c27b0" name="Duration (min)" radius={[4, 4, 0, 0]} />
						</BarChart>
					</ResponsiveContainer>
				</Box>
			</CardContent>
		</Card>
	);
};

export default SessionsDurationChart;
