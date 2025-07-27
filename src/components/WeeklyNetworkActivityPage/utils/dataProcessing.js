/**
 * Data processing utilities for network activity data
 */

/**
 * Enhance session data with persistent state information to show real traffic totals
 * @param {Array} sessionData - Array of daily session data from storage
 * @param {Object} persistentState - Current network totals from persistent state
 * @returns {Array} Enhanced daily data with real traffic totals
 */
export const enhanceDataWithPersistentState = (sessionData, persistentState) => {
	console.log('🔄 Enhancing data with persistent state:', {
		sessionDataLength: sessionData?.length || 0,
		hasPersistentState: !!persistentState?.persistent_state,
		persistentAdapters: Object.keys(persistentState?.persistent_state?.persistent_state || {})
	});

	if (!sessionData || sessionData.length === 0) {
		console.log('⚠️ No session data available');
		return [];
	}

	// If no persistent state, return session data as-is but with enhanced format
	if (!persistentState || !persistentState.persistent_state?.persistent_state) {
		console.log('⚠️ No persistent state, returning enhanced session data');
		return sessionData.map(day => ({
			...day,
			hasRealTimeData: false,
			real_incoming_bytes: day.total_incoming_bytes,
			real_outgoing_bytes: day.total_outgoing_bytes,
			session_incoming_bytes: day.total_incoming_bytes,
			session_outgoing_bytes: day.total_outgoing_bytes
		}));
	}

	const adapters = persistentState.persistent_state.persistent_state;
	const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD format
	
	// Calculate current cumulative totals from all adapters
	let currentCumulativeIncoming = 0;
	let currentCumulativeOutgoing = 0;
	for (const adapter of Object.values(adapters)) {
		currentCumulativeIncoming += adapter.cumulative_incoming_bytes || 0;
		currentCumulativeOutgoing += adapter.cumulative_outgoing_bytes || 0;
	}
	
	console.log('📊 Current cumulative totals:', {
		incoming: currentCumulativeIncoming,
		outgoing: currentCumulativeOutgoing,
		incomingMB: Math.round(currentCumulativeIncoming / (1024 * 1024)),
		outgoingMB: Math.round(currentCumulativeOutgoing / (1024 * 1024))
	});

	// Calculate total historical session data (all past days combined)
	const totalHistoricalIncoming = sessionData.reduce((sum, day) => {
		return day.date !== today ? sum + day.total_incoming_bytes : sum;
	}, 0);
	const totalHistoricalOutgoing = sessionData.reduce((sum, day) => {
		return day.date !== today ? sum + day.total_outgoing_bytes : sum;
	}, 0);

	console.log('📈 Historical totals:', {
		historicalIncoming: totalHistoricalIncoming,
		historicalOutgoing: totalHistoricalOutgoing,
		historicalIncomingMB: Math.round(totalHistoricalIncoming / (1024 * 1024)),
		historicalOutgoingMB: Math.round(totalHistoricalOutgoing / (1024 * 1024))
	});
	
	return sessionData.map(day => {
		let dayRealIncoming = day.total_incoming_bytes;
		let dayRealOutgoing = day.total_outgoing_bytes;
		let hasRealTimeData = false;
		
		if (day.date === today) {
			// For today, use the full cumulative totals which includes all historical data
			dayRealIncoming = currentCumulativeIncoming;
			dayRealOutgoing = currentCumulativeOutgoing;
			hasRealTimeData = true;
			
			console.log(`📅 Today (${today}) enhanced with real-time data:`, {
				sessionIncoming: day.total_incoming_bytes,
				sessionOutgoing: day.total_outgoing_bytes,
				realIncoming: dayRealIncoming,
				realOutgoing: dayRealOutgoing,
				incomingMB: Math.round(dayRealIncoming / (1024 * 1024)),
				outgoingMB: Math.round(dayRealOutgoing / (1024 * 1024))
			});
		} else {
			console.log(`📅 Past day (${day.date}) using session data:`, {
				incoming: day.total_incoming_bytes,
				outgoing: day.total_outgoing_bytes,
				incomingMB: Math.round(day.total_incoming_bytes / (1024 * 1024)),
				outgoingMB: Math.round(day.total_outgoing_bytes / (1024 * 1024))
			});
		}
		
		// Create enhanced day object
		const enhanced = {
			...day,
			// Add real-time data fields
			real_incoming_bytes: dayRealIncoming,
			real_outgoing_bytes: dayRealOutgoing,
			hasRealTimeData: hasRealTimeData,
			// Keep original session data
			session_incoming_bytes: day.total_incoming_bytes,
			session_outgoing_bytes: day.total_outgoing_bytes,
			// Use real-time data if available, otherwise session data
			total_incoming_bytes: dayRealIncoming,
			total_outgoing_bytes: dayRealOutgoing,
		};
		
		return enhanced;
	});
};

/**
 * Calculate total statistics from network data
 * @param {Array} data - Array of daily network data
 * @returns {Object} Total statistics object
 */
export const calculateTotalStats = (data) => {
	return data.reduce((acc, day) => ({
		totalIncoming: acc.totalIncoming + day.total_incoming_bytes,
		totalOutgoing: acc.totalOutgoing + day.total_outgoing_bytes,
		totalDuration: acc.totalDuration + day.total_duration,
		uniqueHosts: Math.max(acc.uniqueHosts, day.unique_hosts),
		uniqueServices: Math.max(acc.uniqueServices, day.unique_services),
		totalSessions: acc.totalSessions + day.sessions.length
	}), {
		totalIncoming: 0,
		totalOutgoing: 0,
		totalDuration: 0,
		uniqueHosts: 0,
		uniqueServices: 0,
		totalSessions: 0
	});
};

/**
 * Prepare chart data from network data (now with real-time enhancement)
 * @param {Array} networkData - Array of daily network data (enhanced)
 * @returns {Array} Formatted chart data
 */
export const prepareChartData = (networkData) => {
	return networkData.map(day => ({
		date: day.date,
		incoming: Math.round(day.total_incoming_bytes / (1024 * 1024)), // Convert to MB (using real-time data)
		outgoing: Math.round(day.total_outgoing_bytes / (1024 * 1024)), // Convert to MB (using real-time data)
		sessions: day.sessions.length,
		duration: Math.round(day.total_duration / 60), // Convert to minutes
		hosts: day.unique_hosts,
		services: day.unique_services,
		// Add indicators for data source
		hasRealTimeData: day.hasRealTimeData || false,
		sessionIncoming: Math.round((day.session_incoming_bytes || 0) / (1024 * 1024)),
		sessionOutgoing: Math.round((day.session_outgoing_bytes || 0) / (1024 * 1024)),
	}));
};

/**
 * Prepare pie chart data from total statistics
 * @param {Object} totalStats - Total statistics object
 * @returns {Array} Pie chart data array
 */
export const preparePieData = (totalStats) => {
	return [
		{ name: 'Incoming', value: totalStats.totalIncoming, color: '#2196f3' },
		{ name: 'Outgoing', value: totalStats.totalOutgoing, color: '#ff9800' }
	];
};
