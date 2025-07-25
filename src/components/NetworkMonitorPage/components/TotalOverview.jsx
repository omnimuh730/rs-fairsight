import React from 'react';
import { Box, Typography } from '@mui/material';
import { Dashboard } from '@mui/icons-material';
import StatsCards from './StatsCards';
import AdapterList from './AdapterList';

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
	
	return (
		<Box>
			<Typography variant="h5" gutterBottom>
				<Dashboard sx={{ mr: 1, verticalAlign: 'middle' }} />
				Network Overview
			</Typography>
			
			<Box sx={{ mb: 3 }}>
				<StatsCards
					totalAdapters={totalAdapters}
					activeAdapters={activeAdapters.length}
					monitoringCount={monitoringCount}
					totalStats={totalStats}
				/>
			</Box>

			<AdapterList adapters={adapters} />
		</Box>
	);
};

export default TotalOverview;
