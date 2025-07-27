import React from 'react';
import { Container, Alert, Typography } from '@mui/material';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import dayjs from 'dayjs';

// Components
import PageHeader from './components/PageHeader';
import NetworkDataSync from './components/NetworkDataSync';
import DateRangeSelector from './components/DateRangeSelector';
import SummaryStatistics from './components/SummaryStatistics';
import AnalyticsSection from './components/AnalyticsSection';
import DailyDetailsSection from './components/DailyDetailsSection';

// Hooks and Utils
import { useWeeklyNetworkData } from './hooks/useWeeklyNetworkData';
import { prepareChartData, preparePieData } from './utils/dataProcessing';

/**
 * Weekly Network Activity Page - Refactored with smaller components
 */
const WeeklyNetworkActivityPage = () => {
	const {
		startDate,
		endDate,
		networkData,
		rawSessionData,
		persistentStateData,
		loading,
		error,
		totalStats,
		setStartDate,
		setEndDate,
		fetchNetworkData
	} = useWeeklyNetworkData(
		dayjs().subtract(7, 'day'),
		dayjs().add(1, 'day') // Include tomorrow to ensure we get today's data regardless of timezone
	);

	// Prepare chart data
	const chartData = prepareChartData(networkData);
	const pieData = preparePieData(totalStats);

	return (
		<LocalizationProvider dateAdapter={AdapterDayjs}>
			<Container maxWidth="xl" sx={{ 
				py: { xs: 3, md: 4 }, 
				px: { xs: 2, md: 3 },
				background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)',
				minHeight: '100vh'
			}}>
				<PageHeader />

				<NetworkDataSync onDataUpdate={(data) => {
					console.log('Network data sync update:', data);
					// Refresh the weekly data when real-time data changes significantly
					if (persistentStateData) {
						const currentTotal = data.combined_totals.total_incoming_bytes + data.combined_totals.total_outgoing_bytes;
						const previousTotal = persistentStateData.combined_totals?.total_incoming_bytes + persistentStateData.combined_totals?.total_outgoing_bytes || 0;
						const diff = Math.abs(currentTotal - previousTotal);
						
						// If there's a significant change (more than 1MB), refresh the data
						if (diff > 1024 * 1024) {
							console.log('📊 Significant traffic change detected, refreshing weekly data...');
							fetchNetworkData();
						}
					}
				}} />

				<DateRangeSelector
					startDate={startDate}
					endDate={endDate}
					onStartDateChange={setStartDate}
					onEndDateChange={setEndDate}
					onFetchData={fetchNetworkData}
					loading={loading}
					dataCount={networkData.length}
				/>

				{error && (
					<Alert 
						severity="error" 
						sx={{ 
							mb: 4, 
							borderRadius: 3,
							boxShadow: '0 4px 20px rgba(244, 67, 54, 0.15)',
							border: '1px solid rgba(244, 67, 54, 0.2)'
						}}
					>
						<Typography variant="body1" fontWeight={500}>{error}</Typography>
					</Alert>
				)}

				<SummaryStatistics totalStats={totalStats} />

				<AnalyticsSection chartData={chartData} pieData={pieData} />

				<DailyDetailsSection networkData={networkData} />
			</Container>
		</LocalizationProvider>
	);
};

export default WeeklyNetworkActivityPage;
