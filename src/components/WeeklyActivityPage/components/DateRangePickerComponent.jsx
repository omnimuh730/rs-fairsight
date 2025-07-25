import React from 'react';
import { DateRangePicker } from 'react-date-range';
import { Box, Paper } from '@mui/material';
import { getStaticRanges } from '../utils/dateHelpers';
import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';

const DateRangePickerComponent = ({ dateRange, onChange, theme }) => {
	return (
		<Box display="flex" justifyContent="center" mb={4}>
			<Paper elevation={3} sx={{ p: { xs: 1, sm: 2 }, width: 'fit-content' }}>
				<DateRangePicker
					ranges={dateRange}
					onChange={onChange}
					showDateDisplay={false}
					direction="vertical"
					months={1}
					rangeColors={[theme.palette.primary.main]}
					inputRanges={[]}
					staticRanges={getStaticRanges(theme)}
				/>
			</Paper>
		</Box>
	);
};

export default DateRangePickerComponent;
