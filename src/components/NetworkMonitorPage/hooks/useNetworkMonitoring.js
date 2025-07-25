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

	// Initialize monitoring states
	useEffect(() => {
		const initializeMonitoringStates = async () => {
			const states = {};
			for (const adapter of adapters) {
				try {
					const isMonitoring = await invoke('is_network_monitoring', { adapterName: adapter.name });
					states[adapter.name] = isMonitoring;
				} catch (err) {
					states[adapter.name] = false;
				}
			}
			setMonitoringStates(states);
		};

		if (adapters.length > 0) {
			initializeMonitoringStates();
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

	const stopMonitoring = async (adapterName) => {
		try {
			await invoke('stop_network_monitoring', { adapterName });
			setMonitoringStates(prev => ({
				...prev,
				[adapterName]: false
			}));
			// Clear stats for this adapter
			setNetworkStats(prev => {
				const newStats = { ...prev };
				delete newStats[adapterName];
				return newStats;
			});
		} catch (err) {
			throw new Error(`Failed to stop monitoring: ${err.toString()}`);
		}
	};

	return {
		monitoringStates,
		networkStats,
		startMonitoring,
		stopMonitoring
	};
};
