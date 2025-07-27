import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { calculateTotalStats, enhanceDataWithPersistentState } from '../utils/dataProcessing';

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
	const [enhancedNetworkData, setEnhancedNetworkData] = useState([]);
	const [persistentStateData, setPersistentStateData] = useState(null);
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
			
			console.log('🔍 Fetching network data for date range:', { startDateStr, endDateStr });
			
			// Fetch both session data and current network totals
			const [sessionData, currentTotals] = await Promise.all([
				invoke('get_network_history', {
					startDate: startDateStr,
					endDate: endDateStr
				}),
				invoke('get_current_network_totals')
			]);
			
			console.log('📊 Raw session data received:', sessionData);
			console.log('📊 Current totals received:', currentTotals);
			
			setNetworkData(sessionData);
			setPersistentStateData(currentTotals);
			
			// If no session data but we have current totals, create a synthetic today entry
			let dataToProcess = sessionData;
			if ((!sessionData || sessionData.length === 0) && currentTotals && currentTotals.today_sessions) {
				const today = new Date().toISOString().split('T')[0];
				console.log('⚠️ No session data found, creating synthetic entry for today:', today);
				dataToProcess = [currentTotals.today_sessions];
			}
			
			// Enhance session data with persistent state information
			const enhanced = enhanceDataWithPersistentState(dataToProcess, currentTotals);
			setEnhancedNetworkData(enhanced);
			
			console.log('✨ Enhanced data created:', enhanced);
			
			// Calculate total stats from enhanced data
			const stats = calculateTotalStats(enhanced);
			setTotalStats(stats);
			
			console.log('� Calculated total stats:', stats);
			
			console.log('📊 Final data summary:', {
				sessions: sessionData.length,
				enhanced: enhanced.length,
				persistent_adapters: Object.keys(currentTotals.persistent_state?.persistent_state || {}).length,
				totalIncomingMB: Math.round(stats.totalIncoming / (1024 * 1024)),
				totalOutgoingMB: Math.round(stats.totalOutgoing / (1024 * 1024))
			});
			
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
		networkData: enhancedNetworkData, // Return enhanced data instead of raw session data
		rawSessionData: networkData, // Keep raw session data available
		persistentStateData,
		loading,
		error,
		totalStats,
		setStartDate,
		setEndDate,
		fetchNetworkData
	};
};
