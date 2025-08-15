import React, { useState, useMemo } from 'react';
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
import { format } from 'date-fns';

import { useNetworkAdapters, useNetworkMonitoring } from './hooks/useNetworkMonitoring';
import { useDailySummary } from './hooks/useDailySummary';
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
		lifetimeStats,
		unexpectedShutdown,
		refreshLifetimeStats,
	} = useNetworkMonitoring(adapters);
	const todayString = useMemo(() => format(new Date(), 'yyyy-MM-dd'), []);
	const { summary: todaySummary, loading: summaryLoading, error: summaryError } = useDailySummary(todayString);

	const handleAdapterSelect = (adapterName) => {
		const adapterIndex = adapters.findIndex(adapter => adapter.name === adapterName);
		setSelectedTab(adapterIndex + 1); // +1 because index 0 is "Total"
	};

	const handleTabChange = (event, newValue) => {
		setSelectedTab(newValue);
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
					Monitor network traffic across your network adapters. All active adapters are automatically monitored with intelligent packet deduplication to prevent duplicate traffic counting from overlapping interfaces.
				</Typography>

				{unexpectedShutdown && (
					<Alert severity="warning" sx={{ mb: 2 }}>
						Previous session ended unexpectedly - some network data may have been lost. 
						The app is now tracking from the last saved state.
					</Alert>
				)}

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
									lifetimeStats={lifetimeStats}
									unexpectedShutdown={unexpectedShutdown}
									onRefreshLifetimeStats={refreshLifetimeStats}
									todaySummary={todaySummary}
								/>
							) : (
								<AdapterDetails
									adapter={adapters[selectedTab - 1]}
									isMonitoring={monitoringStates[adapters[selectedTab - 1]?.name] || false}
									stats={networkStats[adapters[selectedTab - 1]?.name]}
									lifetimeState={lifetimeStats[adapters[selectedTab - 1]?.name]}
									todaySummary={todaySummary}
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
