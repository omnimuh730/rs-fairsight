/**
 * Data processing utilities for network activity data
 */

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
 * Prepare chart data from network data
 * @param {Array} networkData - Array of daily network data
 * @returns {Array} Formatted chart data
 */
export const prepareChartData = (networkData) => {
	return networkData.map(day => ({
		date: day.date,
		incoming: Math.round(day.total_incoming_bytes / (1024 * 1024)), // Convert to MB
		outgoing: Math.round(day.total_outgoing_bytes / (1024 * 1024)), // Convert to MB
		sessions: day.sessions.length,
		duration: Math.round(day.total_duration / 60), // Convert to minutes
		hosts: day.unique_hosts,
		services: day.unique_services
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
