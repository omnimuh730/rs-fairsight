import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

export const useNetworkAdapters = () => {
	const [adapters, setAdapters] = useState([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState(null);

	const fetchNetworkAdapters = async () => {
		try {
			setLoading(true);
			setError(null);
			const result = await invoke('get_network_adapters_command');
			setAdapters(result);
			return result;
		} catch (err) {
			setError(err.toString());
			console.error('Failed to fetch network adapters:', err);
			return [];
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		fetchNetworkAdapters();
	}, []);

	return {
		adapters,
		loading,
		error,
		refetch: fetchNetworkAdapters
	};
};

export const useNetworkMonitoring = (adapters) => {
	const [monitoringStates, setMonitoringStates] = useState({});
	const [networkStats, setNetworkStats] = useState({});
	const pollIntervalRef = useRef(null);

	// Initialize monitoring states and auto-start monitoring for all adapters
	useEffect(() => {
		const initializeAndStartMonitoring = async () => {
			const states = {};
			for (const adapter of adapters) {
				try {
					// Check if already monitoring
					const isMonitoring = await invoke('is_network_monitoring', { adapterName: adapter.name });
					if (!isMonitoring) {
						// Auto-start monitoring for this adapter
						try {
							await invoke('start_network_monitoring', { adapterName: adapter.name });
							states[adapter.name] = true;
							console.log(`Auto-started monitoring for adapter: ${adapter.name}`);
						} catch (startErr) {
							console.warn(`Failed to auto-start monitoring for ${adapter.name}:`, startErr);
							states[adapter.name] = false;
						}
					} else {
						states[adapter.name] = true;
					}
				} catch (err) {
					console.warn(`Failed to check monitoring state for ${adapter.name}:`, err);
					states[adapter.name] = false;
				}
			}
			setMonitoringStates(states);
		};

		if (adapters.length > 0) {
			initializeAndStartMonitoring();
		}
	}, [adapters]);

	// Poll for stats when any adapter is being monitored
	useEffect(() => {
		const activeMonitoring = Object.values(monitoringStates).some(state => state);
		
		if (activeMonitoring && !pollIntervalRef.current) {
			pollIntervalRef.current = setInterval(pollNetworkStats, 1000);
		} else if (!activeMonitoring && pollIntervalRef.current) {
			clearInterval(pollIntervalRef.current);
			pollIntervalRef.current = null;
		}

		return () => {
			if (pollIntervalRef.current) {
				clearInterval(pollIntervalRef.current);
			}
		};
	}, [monitoringStates, adapters]);

	const pollNetworkStats = async () => {
		for (const adapter of adapters) {
			if (monitoringStates[adapter.name]) {
				try {
					const stats = await invoke('get_network_stats', { adapterName: adapter.name });
					setNetworkStats(prev => ({
						...prev,
						[adapter.name]: stats
					}));
				} catch (err) {
					console.error(`Failed to get stats for ${adapter.name}:`, err);
				}
			}
		}
	};

	const startMonitoring = async (adapterName) => {
		try {
			await invoke('start_network_monitoring', { adapterName });
			setMonitoringStates(prev => ({
				...prev,
				[adapterName]: true
			}));
		} catch (err) {
			throw new Error(`Failed to start monitoring: ${err.toString()}`);
		}
	};

	return {
		monitoringStates,
		networkStats,
		startMonitoring
	};
};
