import React from 'react';
import { Card, CardContent, Typography, Box } from '@mui/material';
import {
	LineChart,
	Line,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	Legend,
	ResponsiveContainer
} from 'recharts';

/**
 * Daily Traffic Line Chart Component
 */
const DailyTrafficChart = ({ chartData }) => {
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
			<CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column', width: '100%' }}>
				<Typography variant="h6" gutterBottom sx={{ fontWeight: 600, mb: 2, fontSize: { xs: '1rem', md: '1.25rem' } }}>
					Daily Network Traffic (MB)
				</Typography>
				<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
					<ResponsiveContainer width="100%" height="100%">
						<LineChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 20 }}>
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
								formatter={(value) => [`${value} MB`, '']} 
								contentStyle={{
									backgroundColor: '#fff',
									border: '1px solid #ddd',
									borderRadius: 8,
									boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
								}}
							/>
							<Legend />
							<Line
								type="monotone"
								dataKey="incoming"
								stroke="#2196f3"
								strokeWidth={3}
								name="Incoming"
								dot={{ fill: '#2196f3', strokeWidth: 2, r: 4 }}
								activeDot={{ r: 6, stroke: '#2196f3', strokeWidth: 2 }}
							/>
							<Line
								type="monotone"
								dataKey="outgoing"
								stroke="#ff9800"
								strokeWidth={3}
								name="Outgoing"
								dot={{ fill: '#ff9800', strokeWidth: 2, r: 4 }}
								activeDot={{ r: 6, stroke: '#ff9800', strokeWidth: 2 }}
							/>
						</LineChart>
					</ResponsiveContainer>
				</Box>
			</CardContent>
		</Card>
	);
};

export default DailyTrafficChart;
