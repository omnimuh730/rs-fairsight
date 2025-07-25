import { useState, useEffect } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { parseLog, getDatesInRange } from '../utils/logParser';

export const useActivityData = (dateRange) => {
	const [aggregateTimeList, setAggregateTimeList] = useState([]);
	const [isLoading, setIsLoading] = useState(false);

	const aggregateDaysReport = async (timeList) => {
		setIsLoading(true);
		setAggregateTimeList([]); // Clear previous data immediately
		try {
			if (!Array.isArray(timeList) || timeList.length === 0) {
				console.log("Time list is empty or invalid.");
				setAggregateTimeList([]);
				setIsLoading(false);
				return;
			}
			
			console.log("Fetching logs for dates:", timeList);
			const data = await invoke("aggregate_week_activity_logs", { dataList: timeList });
			console.log("Raw data received:", data);

			if (!Array.isArray(data)) {
				throw new Error("Received invalid data structure from backend.");
			}

			let activityReport = [];
			for (let i = 0; i < data.length; i++) {
				const dailyActivity = parseLog(data[i]);
				activityReport.push(dailyActivity);
			}
			console.log("Parsed activity report:", activityReport);
			setAggregateTimeList(activityReport);
		} catch (error) {
			console.error("Error fetching or processing aggregate_week_activity_logs:", error);
			setAggregateTimeList([]);
		} finally {
			setIsLoading(false);
		}
	};

	// Fetch data when the component mounts or dateRange changes
	useEffect(() => {
		if (dateRange[0].startDate && dateRange[0].endDate) {
			const dates = getDatesInRange(dateRange[0].startDate, dateRange[0].endDate);
			aggregateDaysReport(dates);
		}
	}, [dateRange]);

	return {
		aggregateTimeList,
		isLoading,
		refetch: () => {
			if (dateRange[0].startDate && dateRange[0].endDate) {
				const dates = getDatesInRange(dateRange[0].startDate, dateRange[0].endDate);
				aggregateDaysReport(dates);
			}
		}
	};
};
