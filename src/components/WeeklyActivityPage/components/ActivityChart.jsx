import React from 'react';
import { Box, Paper, Typography } from '@mui/material';
import { LineChart } from '@mui/x-charts/LineChart';
import { ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR } from '../../../utils/colorSetting';
import { getDatesInRange } from '../utils/logParser';

const ActivityChart = ({ aggregateTimeList, dateRange, isLoading }) => {
	// Prepare chart data, ensuring robustness against parsing errors
	const chartSeriesData = aggregateTimeList.map(item => {
		try {
			return JSON.parse(item);
		} catch (e) {
			console.error("Failed to parse item for chart:", item, e);
			// Return default zero values if parsing fails for an item
			return { active: 0, inactive: 0, notrun: 0 };
		}
	});

	// Generate x-axis labels based on the actual dates used for fetching
	const xAxisLabels = getDatesInRange(dateRange[0].startDate, dateRange[0].endDate)
		.map(dateStr => {
			const date = new Date(dateStr + 'T00:00:00'); // Ensure correct date parsing
			return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
		});

	// Ensure chart data length matches x-axis labels length
	const dataLength = Math.min(chartSeriesData.length, xAxisLabels.length);

	const chartData = {
		series: [
			{
				id: 'active',
				data: chartSeriesData.slice(0, dataLength).map(item => (item.active || 0) / 3600),
				label: 'Active',
				area: true,
				stack: 'total',
				color: ACTIVE_COLOR || '#4caf50',
				curve: 'monotoneX',
				showMark: false,
			},
			{
				id: 'inactive',
				data: chartSeriesData.slice(0, dataLength).map(item => (item.inactive || 0) / 3600),
				label: 'Inactive',
				area: true,
				stack: 'total',
				color: INACTIVE_COLOR || '#ff9800',
				curve: 'monotoneX',
				showMark: false,
			},
			{
				id: 'notrun',
				data: chartSeriesData.slice(0, dataLength).map(item => (item.notrun || 0) / 3600),
				label: 'Not Run',
				area: true,
				stack: 'total',
				color: NOTRUN_COLOR || '#9e9e9e',
				curve: 'monotoneX',
				showMark: false,
			},
		],
		xAxis: [{
			data: xAxisLabels.slice(0, dataLength),
			scaleType: 'point',
			id: 'dates',
		}],
		yAxis: [{ label: 'Hours' }],
		height: 300,
		sx: {
			'.MuiLineElement-root': { strokeWidth: 2 },
			'.MuiAreaElement-root': { opacity: 0.6 },
		},
		slotProps: {
			legend: {
				hidden: true,
			},
		},
	};

	if (isLoading) {
		return (
			<Box display="flex" justifyContent="center">
				<Typography>Loading chart data...</Typography>
			</Box>
		);
	}

	if (aggregateTimeList.length > 0 && xAxisLabels.length > 0 && dataLength > 0) {
		return (
			<Box display="flex" justifyContent="center">
				<Paper elevation={3} sx={{ p: { xs: 1, sm: 2, md: 3 }, width: '100%', maxWidth: 1000 }}>
					<LineChart
						{...chartData}
						margin={{ top: 20, right: 30, bottom: 50, left: 60 }}
						tooltip={{ trigger: 'axis' }}
					/>
				</Paper>
			</Box>
		);
	}

	if (!isLoading) {
		return (
			<Box display="flex" justifyContent="center">
				<Typography>No activity data available for the selected range.</Typography>
			</Box>
		);
	}

	return null;
};

export default ActivityChart;
