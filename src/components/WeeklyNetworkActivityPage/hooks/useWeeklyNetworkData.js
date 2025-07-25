import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { calculateTotalStats } from '../utils/dataProcessing';

/**
 * Custom hook for managing weekly network activity data
 * @param {Object} initialStartDate - Initial start date (dayjs object)
 * @param {Object} initialEndDate - Initial end date (dayjs object)
 * @returns {Object} Hook state and functions
 */
export const useWeeklyNetworkData = (initialStartDate, initialEndDate) => {
	const [startDate, setStartDate] = useState(initialStartDate);
	const [endDate, setEndDate] = useState(initialEndDate);
	const [networkData, setNetworkData] = useState([]);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState(null);
	const [totalStats, setTotalStats] = useState({
		totalIncoming: 0,
		totalOutgoing: 0,
		totalDuration: 0,
		uniqueHosts: 0,
		uniqueServices: 0,
		totalSessions: 0
	});

	const fetchNetworkData = async () => {
		setLoading(true);
		setError(null);
		
		try {
			const startDateStr = startDate.format('YYYY-MM-DD');
			const endDateStr = endDate.format('YYYY-MM-DD');
			
			const data = await invoke('get_network_history', {
				startDate: startDateStr,
				endDate: endDateStr
			});
			
			setNetworkData(data);
			const stats = calculateTotalStats(data);
			setTotalStats(stats);
		} catch (err) {
			setError(`Failed to fetch network data: ${err}`);
			console.error('Error fetching network data:', err);
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		fetchNetworkData();
	}, []);

	return {
		startDate,
		endDate,
		networkData,
		loading,
		error,
		totalStats,
		setStartDate,
		setEndDate,
		fetchNetworkData
	};
};
