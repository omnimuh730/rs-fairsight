import React, { useState, useEffect, useRef } from 'react';
import {
	Card,
	CardContent,
	Typography,
	Box,
	Button,
	List,
	ListItem,
	ListItemText,
	ListItemButton,
	Chip,
	Alert,
	CircularProgress,
	Container,
	Paper,
	Divider,
	Tabs,
	Tab,
	Grid,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	LinearProgress,
	IconButton,
	Tooltip
} from '@mui/material';
import {
	NetworkCheck,
	Wifi,
	WifiOff,
	Computer,
	Router,
	CheckCircle,
	Error as ErrorIcon,
	Dashboard,
	Speed,
	Storage,
	PlayArrow,
	Stop,
	Refresh,
	TrendingUp,
	TrendingDown,
	Language,
	Security
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';

const NetworkMonitorPage = () => {
	const [adapters, setAdapters] = useState([]);
	const [selectedTab, setSelectedTab] = useState(0); // 0 for "Total", 1+ for individual adapters
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState(null);
	const [monitoringStates, setMonitoringStates] = useState({}); // Track monitoring state for each adapter
	const [networkStats, setNetworkStats] = useState({}); // Store stats for each adapter
	const pollIntervalRef = useRef(null);

	useEffect(() => {
		fetchNetworkAdapters();
		return () => {
			// Cleanup polling when component unmounts
			if (pollIntervalRef.current) {
				clearInterval(pollIntervalRef.current);
			}
		};
	}, []);

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
	}, [monitoringStates]);

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

	const fetchNetworkAdapters = async () => {
		try {
			setLoading(true);
			setError(null);
			const result = await invoke('get_network_adapters_command');
			setAdapters(result);
			
			// Check monitoring states for all adapters
			const states = {};
			for (const adapter of result) {
				try {
					const isMonitoring = await invoke('is_network_monitoring', { adapterName: adapter.name });
					states[adapter.name] = isMonitoring;
				} catch (err) {
					states[adapter.name] = false;
				}
			}
			setMonitoringStates(states);
		} catch (err) {
			setError(err.toString());
			console.error('Failed to fetch network adapters:', err);
		} finally {
			setLoading(false);
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
			setError(`Failed to start monitoring: ${err.toString()}`);
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
			setError(`Failed to stop monitoring: ${err.toString()}`);
		}
	};

	const handleAdapterSelect = (adapterName) => {
		const adapterIndex = adapters.findIndex(adapter => adapter.name === adapterName);
		setSelectedTab(adapterIndex + 1); // +1 because index 0 is "Total"
	};

	const handleTabChange = (event, newValue) => {
		setSelectedTab(newValue);
	};

	const getShortAdapterName = (adapter) => {
		// Extract a short name from the adapter name
		let shortName = adapter.name;
		
		// Remove common prefixes and suffixes
		shortName = shortName.replace(/\\Device\\NPF_/, '');
		shortName = shortName.replace(/^{.*?}$/, 'Adapter');
		
		// For common adapter types, use friendly names
		if (adapter.description) {
			if (adapter.description.toLowerCase().includes('ethernet')) {
				return 'Ethernet';
			}
			if (adapter.description.toLowerCase().includes('wifi') || adapter.description.toLowerCase().includes('wireless')) {
				return 'WiFi';
			}
			if (adapter.description.toLowerCase().includes('vmware')) {
				return 'VMware';
			}
			if (adapter.description.toLowerCase().includes('loopback')) {
				return 'Loopback';
			}
			if (adapter.description.toLowerCase().includes('wan miniport')) {
				if (adapter.description.includes('IPv6')) return 'WAN6';
				if (adapter.description.includes('IP')) return 'WAN';
				return 'WAN';
			}
		}
		
		// If name is too long, truncate it
		if (shortName.length > 10) {
			return shortName.substring(0, 8) + '..';
		}
		
		return shortName || 'Adapter';
	};

	const getAdapterIcon = (adapter) => {
		if (adapter.is_loopback) {
			return <Computer />;
		}
		if (adapter.is_up) {
			return <Wifi color="primary" />;
		}
		return <WifiOff color="disabled" />;
	};

	const getStatusChip = (adapter) => {
		if (adapter.is_loopback) {
			return <Chip label="Loopback" size="small" variant="outlined" />;
		}
		if (adapter.is_up) {
			return <Chip 
				label="Active" 
				size="small" 
				color="success" 
				icon={<CheckCircle />} 
			/>;
		}
		return <Chip 
			label="Inactive" 
			size="small" 
			color="error" 
			icon={<ErrorIcon />} 
		/>;
	};

	if (loading) {
		return (
			<Container maxWidth="lg">
				<Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
					<CircularProgress />
					<Typography variant="h6" sx={{ ml: 2 }}>
						Loading network adapters...
					</Typography>
				</Box>
			</Container>
		);
	}

	return (
		<Container maxWidth="lg">
			<Box sx={{ py: 2 }}>
				<Typography variant="h4" component="h1" gutterBottom>
					<NetworkCheck sx={{ mr: 1, verticalAlign: 'middle' }} />
					Network Monitor
				</Typography>
				
				<Typography variant="body1" color="text.secondary" paragraph>
					Monitor network traffic across your network adapters. Use the tabs below to view overall statistics or details for individual adapters.
				</Typography>

				{error && (
					<Alert severity="error" sx={{ mb: 2 }}>
						{error}
						<Button onClick={fetchNetworkAdapters} sx={{ ml: 2 }}>
							Retry
						</Button>
					</Alert>
				)}

				{adapters.length === 0 ? (
					<Paper elevation={1} sx={{ p: 3, textAlign: 'center' }}>
						<Router sx={{ fontSize: 60, color: 'text.disabled', mb: 2 }} />
						<Typography variant="h6" color="text.secondary">
							No network adapters found
						</Typography>
						<Button 
							variant="outlined" 
							onClick={fetchNetworkAdapters}
							sx={{ mt: 2 }}
						>
							Refresh
						</Button>
					</Paper>
				) : (
					<Paper elevation={1}>
						<Tabs 
							value={selectedTab} 
							onChange={handleTabChange}
							variant="scrollable"
							scrollButtons="auto"
							sx={{ borderBottom: 1, borderColor: 'divider' }}
						>
							<Tab 
								label="Total" 
								icon={<Dashboard />} 
								sx={{ minWidth: 120 }}
							/>
							{adapters.map((adapter, index) => (
								<Tab
									key={adapter.name}
									label={getShortAdapterName(adapter)}
									icon={getAdapterIcon(adapter)}
									sx={{ minWidth: 120 }}
								/>
							))}
						</Tabs>

						<Box sx={{ p: 3 }}>
							{selectedTab === 0 ? (
								<TotalOverview adapters={adapters} monitoringStates={monitoringStates} networkStats={networkStats} />
							) : (
								<AdapterDetails 
									adapter={adapters[selectedTab - 1]} 
									onSelect={handleAdapterSelect}
									isMonitoring={monitoringStates[adapters[selectedTab - 1]?.name] || false}
									onStartMonitoring={startMonitoring}
									onStopMonitoring={stopMonitoring}
									stats={networkStats[adapters[selectedTab - 1]?.name]}
								/>
							)}
						</Box>
					</Paper>
				)}
			</Box>
		</Container>
	);
};

// Component for the "Total" tab showing overview of all adapters
const TotalOverview = ({ adapters, monitoringStates, networkStats }) => {
	const activeAdapters = adapters.filter(adapter => adapter.is_up && !adapter.is_loopback);
	const totalAdapters = adapters.length;
	const monitoringCount = Object.values(monitoringStates).filter(Boolean).length;
	
	// Calculate total stats across all monitored adapters
	const totalStats = Object.values(networkStats).reduce((acc, stats) => {
		if (!stats) return acc;
		return {
			totalIncoming: acc.totalIncoming + stats.total_incoming_bytes,
			totalOutgoing: acc.totalOutgoing + stats.total_outgoing_bytes,
			totalHosts: acc.totalHosts + stats.network_hosts.length,
			totalServices: acc.totalServices + stats.services.length,
		};
	}, { totalIncoming: 0, totalOutgoing: 0, totalHosts: 0, totalServices: 0 });

	const formatBytes = (bytes) => {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};
	
	return (
		<Box>
			<Typography variant="h5" gutterBottom>
				<Dashboard sx={{ mr: 1, verticalAlign: 'middle' }} />
				Network Overview
			</Typography>
			
			<Grid container spacing={3} sx={{ mb: 3 }}>
				<Grid item xs={12} sm={6} md={3}>
					<Card>
						<CardContent>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Router color="primary" sx={{ mr: 1 }} />
								<Typography variant="h6">Total Adapters</Typography>
							</Box>
							<Typography variant="h3" color="primary">
								{totalAdapters}
							</Typography>
						</CardContent>
					</Card>
				</Grid>
				
				<Grid item xs={12} sm={6} md={3}>
					<Card>
						<CardContent>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<CheckCircle color="success" sx={{ mr: 1 }} />
								<Typography variant="h6">Active</Typography>
							</Box>
							<Typography variant="h3" color="success.main">
								{activeAdapters.length}
							</Typography>
						</CardContent>
					</Card>
				</Grid>
				
				<Grid item xs={12} sm={6} md={3}>
					<Card>
						<CardContent>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Speed color="info" sx={{ mr: 1 }} />
								<Typography variant="h6">Monitoring</Typography>
							</Box>
							<Typography variant="h3" color="info.main">
								{monitoringCount}
							</Typography>
							<Typography variant="caption" color="text.secondary">
								Active Sessions
							</Typography>
						</CardContent>
					</Card>
				</Grid>
				
				<Grid item xs={12} sm={6} md={3}>
					<Card>
						<CardContent>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Storage color="warning" sx={{ mr: 1 }} />
								<Typography variant="h6">Total Data</Typography>
							</Box>
							<Typography variant="h3" color="warning.main">
								{formatBytes(totalStats.totalIncoming + totalStats.totalOutgoing)}
							</Typography>
							<Typography variant="caption" color="text.secondary">
								↓ {formatBytes(totalStats.totalIncoming)} ↑ {formatBytes(totalStats.totalOutgoing)}
							</Typography>
						</CardContent>
					</Card>
				</Grid>
			</Grid>

			<Card>
				<CardContent>
					<Typography variant="h6" gutterBottom>
						Adapter Summary
					</Typography>
					<List>
						{adapters.map((adapter, index) => (
							<React.Fragment key={adapter.name}>
								<ListItem>
									<Box sx={{ mr: 2 }}>
										{adapter.is_loopback ? <Computer /> : 
										 adapter.is_up ? <Wifi color="primary" /> : <WifiOff color="disabled" />}
									</Box>
									<ListItemText
										primary={
											<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
												<Typography variant="subtitle1">
													{adapter.description || adapter.name}
												</Typography>
												{adapter.is_loopback ? (
													<Chip label="Loopback" size="small" variant="outlined" />
												) : adapter.is_up ? (
													<Chip label="Active" size="small" color="success" />
												) : (
													<Chip label="Inactive" size="small" color="error" />
												)}
											</Box>
										}
										secondary={
											adapter.addresses.length > 0 ? 
											`${adapter.addresses.length} address${adapter.addresses.length > 1 ? 'es' : ''}` :
											'No addresses'
										}
									/>
								</ListItem>
								{index < adapters.length - 1 && <Divider />}
							</React.Fragment>
						))}
					</List>
				</CardContent>
			</Card>
		</Box>
	);
};

// Component for individual adapter details
const AdapterDetails = ({ adapter, onSelect, isMonitoring, onStartMonitoring, onStopMonitoring, stats }) => {
	if (!adapter) {
		return (
			<Alert severity="error">
				Adapter not found
			</Alert>
		);
	}

	const formatBytes = (bytes) => {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};

	const formatDuration = (seconds) => {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const secs = seconds % 60;
		return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	};

	const getCountryFlag = (countryCode) => {
		if (!countryCode) return '🌐';
		const codePoints = countryCode
			.toUpperCase()
			.split('')
			.map(char => 127397 + char.charCodeAt());
		return String.fromCodePoint(...codePoints);
	};

	return (
		<Box>
			<Typography variant="h5" gutterBottom>
				<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
					{adapter.is_loopback ? <Computer /> : 
					 adapter.is_up ? <Wifi color="primary" /> : <WifiOff color="disabled" />}
					{adapter.description || adapter.name}
					{adapter.is_loopback ? (
						<Chip label="Loopback" size="small" variant="outlined" />
					) : adapter.is_up ? (
						<Chip label="Active" size="small" color="success" />
					) : (
						<Chip label="Inactive" size="small" color="error" />
					)}
					<Box sx={{ ml: 'auto' }}>
						{isMonitoring ? (
							<Button
								variant="contained"
								color="error"
								startIcon={<Stop />}
								onClick={() => onStopMonitoring(adapter.name)}
							>
								Stop Monitoring
							</Button>
						) : (
							<Button
								variant="contained"
								color="primary"
								startIcon={<PlayArrow />}
								onClick={() => onStartMonitoring(adapter.name)}
								disabled={!adapter.is_up}
							>
								Start Monitoring
							</Button>
						)}
					</Box>
				</Box>
			</Typography>

			{!isMonitoring ? (
				<Grid container spacing={3}>
					<Grid item xs={12} md={6}>
						<Card>
							<CardContent>
								<Typography variant="h6" gutterBottom>
									Adapter Information
								</Typography>
								
								<Box sx={{ mb: 2 }}>
									<Typography variant="body2" color="text.secondary">
										Name
									</Typography>
									<Typography variant="body1" sx={{ fontFamily: 'monospace', wordBreak: 'break-all' }}>
										{adapter.name}
									</Typography>
								</Box>

								{adapter.description && (
									<Box sx={{ mb: 2 }}>
										<Typography variant="body2" color="text.secondary">
											Description
										</Typography>
										<Typography variant="body1">
											{adapter.description}
										</Typography>
									</Box>
								)}

								<Box sx={{ mb: 2 }}>
									<Typography variant="body2" color="text.secondary">
										Status
									</Typography>
									<Box sx={{ display: 'flex', gap: 1, mt: 1 }}>
										<Chip 
											label={adapter.is_up ? "Active" : "Inactive"} 
											color={adapter.is_up ? "success" : "error"}
											size="small"
										/>
										{adapter.is_loopback && (
											<Chip label="Loopback" variant="outlined" size="small" />
										)}
									</Box>
								</Box>
							</CardContent>
						</Card>
					</Grid>

					<Grid item xs={12} md={6}>
						<Card>
							<CardContent>
								<Typography variant="h6" gutterBottom>
									Network Addresses
								</Typography>
								
								{adapter.addresses.length === 0 ? (
									<Typography color="text.secondary">
										No addresses assigned
									</Typography>
								) : (
									<List dense>
										{adapter.addresses.map((address, idx) => (
											<ListItem key={idx} disablePadding>
												<ListItemText
													primary={
														<Typography 
															variant="body2" 
															sx={{ fontFamily: 'monospace' }}
														>
															{address}
														</Typography>
													}
												/>
											</ListItem>
										))}
									</List>
								)}
							</CardContent>
						</Card>
					</Grid>

					<Grid item xs={12}>
						<Alert severity="info">
							Click "Start Monitoring" to begin capturing network traffic for this adapter.
							You'll see real-time statistics, network hosts, and service information.
						</Alert>
					</Grid>
				</Grid>
			) : (
				// Monitoring interface
				<MonitoringInterface adapter={adapter} stats={stats} />
			)}
		</Box>
	);
};

// Monitoring interface component
const MonitoringInterface = ({ adapter, stats }) => {
	const formatBytes = (bytes) => {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};

	const formatDuration = (seconds) => {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const secs = seconds % 60;
		return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	};

	const getCountryFlag = (countryCode) => {
		if (!countryCode) return '🌐';
		try {
			const codePoints = countryCode
				.toUpperCase()
				.split('')
				.map(char => 127397 + char.charCodeAt());
			return String.fromCodePoint(...codePoints);
		} catch {
			return '🌐';
		}
	};

	if (!stats) {
		return (
			<Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: 200 }}>
				<CircularProgress />
				<Typography sx={{ ml: 2 }}>Loading monitoring data...</Typography>
			</Box>
		);
	}

	return (
		<Grid container spacing={3}>
			{/* Stats Cards */}
			<Grid item xs={12}>
				<Grid container spacing={2}>
					<Grid item xs={12} sm={6} md={3}>
						<Card>
							<CardContent>
								<Typography variant="h6" color="primary">
									↓ {formatBytes(stats.total_incoming_bytes)}
								</Typography>
								<Typography variant="body2" color="text.secondary">
									Incoming
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={3}>
						<Card>
							<CardContent>
								<Typography variant="h6" color="secondary">
									↑ {formatBytes(stats.total_outgoing_bytes)}
								</Typography>
								<Typography variant="body2" color="text.secondary">
									Outgoing
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={3}>
						<Card>
							<CardContent>
								<Typography variant="h6">
									{stats.network_hosts.length}
								</Typography>
								<Typography variant="body2" color="text.secondary">
									Hosts
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={3}>
						<Card>
							<CardContent>
								<Typography variant="h6">
									{formatDuration(stats.monitoring_duration)}
								</Typography>
								<Typography variant="body2" color="text.secondary">
									Duration
								</Typography>
							</CardContent>
						</Card>
					</Grid>
				</Grid>
			</Grid>

			{/* Traffic Rate Chart */}
			<Grid item xs={12}>
				<Card>
					<CardContent>
						<Typography variant="h6" gutterBottom>
							Traffic Rate
						</Typography>
						<TrafficChart data={stats.traffic_rate} />
					</CardContent>
				</Card>
			</Grid>

			{/* Network Hosts and Services */}
			<Grid item xs={12}>
				<Card>
					<CardContent>
						<Typography variant="h6" gutterBottom>
							Network Hosts
						</Typography>
						<TableContainer>
							<Table size="small">
								<TableHead>
									<TableRow>
										<TableCell>Host</TableCell>
										<TableCell>Country</TableCell>
										<TableCell align="right">Incoming</TableCell>
										<TableCell align="right">Outgoing</TableCell>
									</TableRow>
								</TableHead>
								<TableBody>
									{stats.network_hosts.slice(0, 10).map((host, index) => (
										<TableRow key={host.ip}>
											<TableCell>
												<Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
													{host.ip}
												</Typography>
												{host.hostname && (
													<Typography variant="caption" color="text.secondary">
														{host.hostname}
													</Typography>
												)}
											</TableCell>
											<TableCell>
												<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
													<span>{getCountryFlag(host.country_code)}</span>
													<Typography variant="body2">
														{host.country || 'Unknown'}
													</Typography>
												</Box>
											</TableCell>
											<TableCell align="right">
												<Typography variant="body2" color="primary">
													{formatBytes(host.incoming_bytes)}
												</Typography>
											</TableCell>
											<TableCell align="right">
												<Typography variant="body2" color="secondary">
													{formatBytes(host.outgoing_bytes)}
												</Typography>
											</TableCell>
										</TableRow>
									))}
								</TableBody>
							</Table>
						</TableContainer>
					</CardContent>
				</Card>
			</Grid>

			<Grid item xs={12}>
				<Card>
					<CardContent>
						<Typography variant="h6" gutterBottom>
							Services
						</Typography>
						<List dense>
							{stats.services.slice(0, 10).map((service, index) => (
								<ListItem key={`${service.protocol}:${service.port}`} disablePadding>
									<ListItemText
										primary={
											<Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
												<Typography variant="body2">
													{service.service_name || `${service.protocol}:${service.port}`}
												</Typography>
												<Typography variant="body2" color="text.secondary">
													{formatBytes(service.bytes)}
												</Typography>
											</Box>
										}
										secondary={
											<LinearProgress
												variant="determinate"
												value={(service.bytes / Math.max(...stats.services.map(s => s.bytes))) * 100}
												sx={{ mt: 0.5 }}
											/>
										}
									/>
								</ListItem>
							))}
						</List>
					</CardContent>
				</Card>
			</Grid>
		</Grid>
	);
};

// Simple traffic chart component (you can replace with a proper charting library)
const TrafficChart = ({ data }) => {
	if (!data || data.length === 0) {
		return (
			<Box sx={{ height: 200, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
				<Typography color="text.secondary">No data available</Typography>
			</Box>
		);
	}

	const maxBytes = Math.max(...data.map(d => Math.max(d.incoming_bytes, d.outgoing_bytes)));
	
	return (
		<Box sx={{ height: 200, display: 'flex', alignItems: 'end', gap: 1, overflow: 'hidden' }}>
			{data.slice(-60).map((point, index) => (
				<Box key={index} sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', minWidth: 3 }}>
					<Box
						sx={{
							width: 3,
							height: Math.max(1, (point.incoming_bytes / maxBytes) * 100),
							backgroundColor: 'primary.main',
							mb: 0.25
						}}
					/>
					<Box
						sx={{
							width: 3,
							height: Math.max(1, (point.outgoing_bytes / maxBytes) * 100),
							backgroundColor: 'secondary.main'
						}}
					/>
				</Box>
			))}
		</Box>
	);
};

export default NetworkMonitorPage;
