import React from 'react';
import { Card, CardContent, Typography, Box, Chip } from '@mui/material';
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
 * Enhanced tooltip for showing both real-time and session data
 */
const CustomTooltip = ({ active, payload, label }) => {
	if (active && payload && payload.length) {
		const data = payload[0].payload;
		return (
			<Box sx={{ 
				bgcolor: 'white', 
				p: 2, 
				border: '1px solid #ddd', 
				borderRadius: 1,
				boxShadow: '0 4px 8px rgba(0,0,0,0.1)'
			}}>
				<Typography variant="subtitle2" gutterBottom>{label}</Typography>
				{data.hasRealTimeData && (
					<Chip label="Real-time Data" color="success" size="small" sx={{ mb: 1 }} />
				)}
				<Typography variant="body2">Incoming: {data.incoming} MB</Typography>
				<Typography variant="body2">Outgoing: {data.outgoing} MB</Typography>
				{data.hasRealTimeData && data.sessionIncoming !== data.incoming && (
					<Box sx={{ mt: 1, pt: 1, borderTop: '1px solid #eee' }}>
						<Typography variant="caption" color="text.secondary">
							Session data: ↓{data.sessionIncoming}MB ↑{data.sessionOutgoing}MB
						</Typography>
					</Box>
				)}
				<Typography variant="body2">Sessions: {data.sessions}</Typography>
				<Typography variant="body2">Duration: {data.duration} min</Typography>
			</Box>
		);
	}
	return null;
};

/**
 * Daily Traffic Line Chart Component - Enhanced with real-time data
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
				<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
					<Typography variant="h6" sx={{ fontWeight: 600, fontSize: { xs: '1rem', md: '1.25rem' } }}>
						Daily Network Traffic (MB)
					</Typography>
					{chartData.some(d => d.hasRealTimeData) && (
						<Chip 
							label="Live Data" 
							color="success" 
							size="small" 
							variant="outlined"
						/>
					)}
				</Box>
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
							<Tooltip content={<CustomTooltip />} />
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
