import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

export const useNetworkAdapters = () => {
	const [adapters, setAdapters] = useState([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState(null);
	const adapterRefreshIntervalRef = useRef(null);
	const isInitialLoadRef = useRef(true);

	const fetchNetworkAdapters = async () => {
		try {
			// Only set loading=true on very first load
			if (isInitialLoadRef.current) {
				setLoading(true);
			}
			setError(null);
			const result = await invoke('get_network_adapters_command');
			
			// Only update state if adapters actually changed to prevent unnecessary re-renders
			if (JSON.stringify(result) !== JSON.stringify(adapters)) {
				setAdapters(result);
			}
			
			return result;
		} catch (err) {
			setError(err.toString());
			console.error('Failed to fetch network adapters:', err);
			return [];
		} finally {
			if (isInitialLoadRef.current) {
				setLoading(false);
				isInitialLoadRef.current = false;
			}
		}
	};

	useEffect(() => {
		fetchNetworkAdapters();

		// Set up periodic adapter discovery to detect VPN/network changes
		// Use longer interval to reduce UI flicker
		adapterRefreshIntervalRef.current = setInterval(() => {
			fetchNetworkAdapters();
		}, 15000); // Check for new adapters every 15 seconds instead of 5

		return () => {
			if (adapterRefreshIntervalRef.current) {
				clearInterval(adapterRefreshIntervalRef.current);
				adapterRefreshIntervalRef.current = null;
			}
		};
	}, []); // Only run once on mount

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
	const [lifetimeStats, setLifetimeStats] = useState({});
	const [unexpectedShutdown, setUnexpectedShutdown] = useState(false);
	const pollIntervalRef = useRef(null);

	// Check for unexpected shutdown on mount
	useEffect(() => {
		const checkShutdown = async () => {
			try {
				const wasUnexpected = await invoke('check_unexpected_shutdown');
				setUnexpectedShutdown(wasUnexpected);
				if (wasUnexpected) {
					console.warn('⚠️  Previous session ended unexpectedly - some data may have been lost');
				}
			} catch (err) {
				console.error('Failed to check shutdown state:', err);
			}
		};
		
		checkShutdown();
	}, []);

	// Load lifetime stats
	useEffect(() => {
		const loadLifetimeStats = async () => {
			try {
				const stats = await invoke('get_lifetime_stats');
				setLifetimeStats(stats);
				console.log('📊 Loaded lifetime network statistics:', stats);
			} catch (err) {
				console.error('Failed to load lifetime stats:', err);
			}
		};
		
		loadLifetimeStats();
	}, []);

	// Initialize monitoring states and auto-start monitoring for ALL adapters with duplicate prevention
	useEffect(() => {
		const initializeAndStartMonitoring = async () => {
			const states = {};
			const currentAdapterNames = adapters.map(a => a.name);
			
			// Stop monitoring for adapters that no longer exist (e.g., VPN disconnected)
			for (const adapterName of Object.keys(monitoringStates)) {
				if (!currentAdapterNames.includes(adapterName)) {
					try {
						await invoke('stop_network_monitoring', { adapterName });
						console.log(`🛑 Stopped monitoring for removed adapter: ${adapterName}`);
					} catch (err) {
						console.warn(`Failed to stop monitoring for removed adapter ${adapterName}:`, err);
					}
				}
			}

			// Start monitoring for all current adapters
			for (const adapter of adapters) {
				try {
					// Check if already monitoring
					const isMonitoring = await invoke('is_network_monitoring', { adapterName: adapter.name });
					if (!isMonitoring && adapter.is_up) {
						// Auto-start monitoring for this active adapter
						try {
							await invoke('start_network_monitoring', { adapterName: adapter.name });
							states[adapter.name] = true;
							console.log(`🚀 Auto-started monitoring for adapter: ${adapter.name} (${adapter.description || 'No description'})`);
						} catch (startErr) {
							console.warn(`Failed to auto-start monitoring for ${adapter.name}:`, startErr);
							states[adapter.name] = false;
						}
					} else {
						states[adapter.name] = isMonitoring && adapter.is_up;
						if (isMonitoring && adapter.is_up) {
							console.log(`✅ Already monitoring adapter: ${adapter.name}`);
						} else if (!adapter.is_up) {
							console.log(`⏸️  Skipping inactive adapter: ${adapter.name}`);
						}
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
		// Reset monitoring states when adapters change completely (like on refresh)
		else if (adapters.length === 0 && Object.keys(monitoringStates).length > 0) {
			setMonitoringStates({});
		}
	}, [adapters]); // React to adapter changes (VPN connect/disconnect, etc.)

	// Poll for stats when any adapter is being monitored
	useEffect(() => {
		const activeMonitoring = Object.values(monitoringStates).some(state => state);
		
		if (activeMonitoring && !pollIntervalRef.current) {
			console.log('📊 Starting stats polling...');
			pollIntervalRef.current = setInterval(pollNetworkStats, 2000); // Reduced from 1000ms to 2000ms
		} else if (!activeMonitoring && pollIntervalRef.current) {
			console.log('⏸️  Stopping stats polling...');
			clearInterval(pollIntervalRef.current);
			pollIntervalRef.current = null;
		}

		return () => {
			if (pollIntervalRef.current) {
				clearInterval(pollIntervalRef.current);
				pollIntervalRef.current = null;
			}
		};
	}, [monitoringStates, adapters]);

	const pollNetworkStats = async () => {
		try {
			for (const adapter of adapters) {
				if (monitoringStates[adapter.name]) {
					try {
						const stats = await invoke('get_network_stats', { adapterName: adapter.name });
						setNetworkStats(prev => ({
							...prev,
							[adapter.name]: stats
						}));
					} catch (err) {
						// Only log errors occasionally to avoid spam
						if (Date.now() % 10000 < 1000) {  // Log roughly every 10 seconds
							console.error(`Failed to get stats for ${adapter.name}:`, err);
						}
					}
				}
			}
		} catch (err) {
			console.error('Error during stats polling:', err);
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
		} catch (err) {
			console.error(`Failed to stop monitoring ${adapterName}:`, err);
			throw err;
		}
	};

	const refreshLifetimeStats = async () => {
		try {
			const stats = await invoke('get_lifetime_stats');
			setLifetimeStats(stats);
			return stats;
		} catch (err) {
			console.error('Failed to refresh lifetime stats:', err);
			throw err;
		}
	};

	return {
		monitoringStates,
		networkStats,
		lifetimeStats,
		unexpectedShutdown,
		startMonitoring,
		stopMonitoring,
		refreshLifetimeStats,
	};
};
