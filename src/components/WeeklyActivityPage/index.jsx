import React, { useState } from 'react';
import { Box, Typography, CssBaseline } from '@mui/material';
import { ThemeProvider } from '@mui/material/styles';

import { useActivityData } from './hooks/useActivityData';
import { getDefaultDateRange } from './utils/dateHelpers';
import { theme } from './utils/theme';
import DateRangePickerComponent from './components/DateRangePickerComponent';
import ActivityChart from './components/ActivityChart';

const WeeklyActivityPage = () => {
	const [dateRange, setDateRange] = useState(getDefaultDateRange());
	const { aggregateTimeList, isLoading } = useActivityData(dateRange);

	const handleDateChange = (ranges) => {
		const { startDate, endDate } = ranges.selection;
		// Update the state to trigger the useEffect hook
		setDateRange([{
			startDate: startDate || new Date(), // Fallback if null
			endDate: endDate || new Date(), // Fallback if null
			key: 'selection',
		}]);
		// Data fetching is now handled by useEffect in the hook
	};

	return (
		<ThemeProvider theme={theme}>
			<CssBaseline />
			<Box sx={{ p: { xs: 1, sm: 2, md: 4 }, bgcolor: 'background.default', minHeight: '100vh' }}>
				<Typography variant="h5" color="textPrimary" gutterBottom align="center" sx={{ mb: 3 }}>
					Activity Dashboard
				</Typography>

				<DateRangePickerComponent 
					dateRange={dateRange}
					onChange={handleDateChange}
					theme={theme}
				/>

				<ActivityChart 
					aggregateTimeList={aggregateTimeList}
					dateRange={dateRange}
					isLoading={isLoading}
				/>
			</Box>
		</ThemeProvider>
	);
};

export default WeeklyActivityPage;