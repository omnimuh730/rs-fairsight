import React, { useState } from 'react';
import {
	Container,
	Typography,
	Box,
	Button,
	Alert,
	CircularProgress,
	Paper,
	Tabs,
	Tab
} from '@mui/material';
import {
	NetworkCheck,
	Router,
	Dashboard
} from '@mui/icons-material';

import { useNetworkAdapters, useNetworkMonitoring } from './hooks/useNetworkMonitoring';
import { getShortAdapterName } from './utils/formatters';
import { getAdapterIcon } from './utils/adapterHelpers';
import TotalOverview from './components/TotalOverview';
import AdapterDetails from './components/AdapterDetails';

const NetworkMonitorPage = () => {
	const [selectedTab, setSelectedTab] = useState(0); // 0 for "Total", 1+ for individual adapters
	
	const { adapters, loading, error, refetch } = useNetworkAdapters();
	const { 
		monitoringStates, 
		networkStats, 
		startMonitoring, 
	} = useNetworkMonitoring(adapters);

	const handleAdapterSelect = (adapterName) => {
		const adapterIndex = adapters.findIndex(adapter => adapter.name === adapterName);
		setSelectedTab(adapterIndex + 1); // +1 because index 0 is "Total"
	};

	const handleTabChange = (event, newValue) => {
		setSelectedTab(newValue);
	};

	const handleStartMonitoring = async (adapterName) => {
		try {
			await startMonitoring(adapterName);
		} catch (err) {
			console.error('Failed to start monitoring:', err);
		}
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
						<Button onClick={refetch} sx={{ ml: 2 }}>
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
							onClick={refetch}
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
								<TotalOverview 
									adapters={adapters} 
									monitoringStates={monitoringStates} 
									networkStats={networkStats} 
								/>
							) : (
								<AdapterDetails 
									adapter={adapters[selectedTab - 1]} 
									isMonitoring={monitoringStates[adapters[selectedTab - 1]?.name] || false}
									onStartMonitoring={handleStartMonitoring}
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

export default NetworkMonitorPage;
