import React, { useState } from 'react';
import { DateRangePicker } from 'react-date-range';
import { invoke } from "@tauri-apps/api/core";
import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';
import Stack from '@mui/material/Stack';
import Box from '@mui/material/Box';
import { LineChart } from '@mui/x-charts/LineChart';

import { ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR } from '../../utils/colorSetting';

const DateRangePickerComponent = () => {
	const [dateRange, setDateRange] = useState([
		{
			startDate: new Date(),
			endDate: new Date(),
			key: 'selection',
		}
	]);

	const [aggregateTimeList, setAggregateTimeList] = useState([]);
	const [selectedItem, setSelectedItem] = useState(null);

	const handleDateChange = async (ranges) => {
		const { startDate, endDate } = ranges.selection;
		setDateRange([ranges.selection]);

		// Generate list of dates
		let currentDate = new Date(startDate);
		let dates = [];

		while (currentDate <= endDate) {
			// Format date as YYYY-MM-DD
			const formattedDate = currentDate.toISOString().split('T')[0];
			dates.push(formattedDate);
			currentDate.setDate(currentDate.getDate() + 1);
		}

		await aggregateDaysReport(dates);
	};

	// Function to parse the log and calculate durations
	const parseLog = (log) => {
		if (log.includes("found")) {
			return JSON.stringify({
				active: 0,
				inactive: 0,
				notrun: 86400
			});
		}
		const lines = log.split("\n").filter((line) => line.trim()); // Split into lines and remove empty ones
		let activeDuration = 0;
		let inactiveDuration = 0;
		let notRunDuration = 0;



		for (let i = 0; i < lines.length; i++) {
			const [state, timeRange] = lines[i].split(": ");
			const [start, end] = timeRange.split(" - ").map((t) => {
				const [h, m, s] = t.split(":").map(Number);
				return h * 3600 + m * 60 + s; // Convert to seconds
			});
			const duration = end - start;

			if (state === "Active") {
				activeDuration += duration;
			} else if (state === "Inactive") {
				inactiveDuration += duration;
			} else if (state === "Not run") {
				notRunDuration += duration;
			}
		}
		return JSON.stringify({
			active: activeDuration,
			inactive: inactiveDuration,
			notrun: notRunDuration
		});
	};

	async function aggregateDaysReport(timeList) {
		try {
			// Ensure timeList is an array of strings
			if (!Array.isArray(timeList)) {
				throw new Error("timeList must be an array");
			}

			// Invoke the Tauri command with timeList as the parameter
			const data = await invoke("aggregate_week_activity_logs", { dataList: timeList });

			// Log the returned data (an array of styled strings)
			let activityReport = [];
			console.log(data);
			for (let i = 0; i < data.length; i++) {
				const dailyActivity = parseLog(data[i]);
				activityReport.push(dailyActivity);
			}

			setAggregateTimeList(activityReport);

			// Example: Update state with the returned data (uncomment and adjust as needed)
			// setTimeData(data);

			return data; // Optionally return the data for further use
			//      setTimeData(processedData); // Update state with parsed data
		} catch (error) {
			console.error("Error fetching sync_time_data:", error);
		}
	}
	// Prepare chart data from aggregateTimeList
	const chartData = {
		series: [
			{
				id: 'active',
				data: aggregateTimeList.map(item => JSON.parse(item).active / 60), // Convert to hours
				label: 'Active',
				area: true,
				stack: 'total',
				color: ACTIVE_COLOR,
				highlightScope: { highlight: 'item' },
			},
			{
				id: 'inactive',
				data: aggregateTimeList.map(item => JSON.parse(item).inactive / 60), // Convert to hours
				label: 'Inactive',
				area: true,
				stack: 'total',
				color: INACTIVE_COLOR,
				highlightScope: { highlight: 'item' },
			},
			{
				id: 'notrun',
				data: aggregateTimeList.map(item => JSON.parse(item).notrun / 60), // Convert to hours
				label: 'Not Run',
				area: true,
				stack: 'total',
				color: NOTRUN_COLOR,
				highlightScope: { highlight: 'item' },
			},
		],
		xAxis: [{
			data: dateRange[0].startDate ?
				Array.from({ length: aggregateTimeList.length }, (_, i) => {
					const date = new Date(dateRange[0].startDate);
					date.setDate(date.getDate() + i);
					return date.toLocaleDateString();
				}) : [],
			scaleType: 'point',
			id: 'dates'
		}],
		height: 400,
	};

	return (
		<Stack direction="column" spacing={2} sx={{ width: '100%' }}>
			<Box>
				<DateRangePicker
					ranges={dateRange}
					onChange={handleDateChange}
				/>
			</Box>

			{aggregateTimeList.length > 0 && (
				<Box sx={{ flexGrow: 1 }}>
					<LineChart
						{...chartData}
						onAreaClick={(event, d) => setSelectedItem(d)}
						onMarkClick={(event, d) => setSelectedItem(d)}
						onLineClick={(event, d) => setSelectedItem(d)}
					/>
				</Box>
			)}

			{selectedItem && (
				<Box>
					<pre>
						Selected Data: {JSON.stringify(selectedItem, null, 2)}
					</pre>
				</Box>
			)}
		</Stack>
	);
};

export default DateRangePickerComponent;
