import React, { useState, useEffect } from 'react';
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
	Grid
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
	Storage
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';

const NetworkMonitorPage = () => {
	const [adapters, setAdapters] = useState([]);
	const [selectedTab, setSelectedTab] = useState(0); // 0 for "Total", 1+ for individual adapters
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState(null);

	useEffect(() => {
		fetchNetworkAdapters();
	}, []);

	const fetchNetworkAdapters = async () => {
		try {
			setLoading(true);
			setError(null);
			const result = await invoke('get_network_adapters_command');
			setAdapters(result);
		} catch (err) {
			setError(err.toString());
			console.error('Failed to fetch network adapters:', err);
		} finally {
			setLoading(false);
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
								<TotalOverview adapters={adapters} />
							) : (
								<AdapterDetails 
									adapter={adapters[selectedTab - 1]} 
									onSelect={handleAdapterSelect}
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
const TotalOverview = ({ adapters }) => {
	const activeAdapters = adapters.filter(adapter => adapter.is_up && !adapter.is_loopback);
	const totalAdapters = adapters.length;
	
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
								0
							</Typography>
							<Typography variant="caption" color="text.secondary">
								Coming Soon
							</Typography>
						</CardContent>
					</Card>
				</Grid>
				
				<Grid item xs={12} sm={6} md={3}>
					<Card>
						<CardContent>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
								<Storage color="warning" sx={{ mr: 1 }} />
								<Typography variant="h6">Data Usage</Typography>
							</Box>
							<Typography variant="h3" color="warning.main">
								--
							</Typography>
							<Typography variant="caption" color="text.secondary">
								Coming Soon
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
const AdapterDetails = ({ adapter, onSelect }) => {
	if (!adapter) {
		return (
			<Alert severity="error">
				Adapter not found
			</Alert>
		);
	}

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
				</Box>
			</Typography>

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
					<Card>
						<CardContent>
							<Typography variant="h6" gutterBottom>
								Network Monitoring
							</Typography>
							<Alert severity="info" sx={{ mb: 2 }}>
								Network traffic monitoring for this adapter will be implemented in the next phase.
								Features will include real-time packet capture, bandwidth monitoring, and traffic analysis.
							</Alert>
							<Box sx={{ display: 'flex', gap: 2 }}>
								<Button variant="contained" disabled>
									Start Monitoring
								</Button>
								<Button variant="outlined" disabled>
									View Statistics
								</Button>
								<Button variant="outlined" disabled>
									Export Data
								</Button>
							</Box>
						</CardContent>
					</Card>
				</Grid>
			</Grid>
		</Box>
	);
};

export default NetworkMonitorPage;
