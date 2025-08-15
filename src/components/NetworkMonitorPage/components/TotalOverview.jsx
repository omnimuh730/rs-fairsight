import React from 'react';
import { Box, Typography } from '@mui/material';
import { Dashboard } from '@mui/icons-material';
import StatsCards from './StatsCards';
import AdapterList from './AdapterList';
import DataPersistenceStatus from './DataPersistenceStatus';

const TotalOverview = ({ 
	adapters, 
	monitoringStates, 
	networkStats, 
	lifetimeStats,
	unexpectedShutdown,
	onRefreshLifetimeStats,
	   todaySummary
}) => {
	const activeAdapters = adapters.filter(adapter => adapter.is_up && !adapter.is_loopback);
	const totalAdapters = adapters.length;
	const monitoringCount = Object.values(monitoringStates).filter(Boolean).length;
	
	const totalStats = {
	       totalIncoming: todaySummary?.total_incoming_bytes || 0,
	       totalOutgoing: todaySummary?.total_outgoing_bytes || 0,
	       totalHosts: todaySummary?.unique_hosts || 0,
	       totalServices: todaySummary?.unique_services || 0,
	   };
	
	return (
		<Box>
			<Typography variant="h5" gutterBottom>
				<Dashboard sx={{ mr: 1, verticalAlign: 'middle' }} />
				Network Overview
			</Typography>
			
			<DataPersistenceStatus
				lifetimeStats={lifetimeStats}
				unexpectedShutdown={unexpectedShutdown}
				onRefreshLifetimeStats={onRefreshLifetimeStats}
				adapters={adapters}
				            todaySummary={todaySummary}
			/>
			
			<Box sx={{ mb: 3 }}>
				<StatsCards
					totalAdapters={totalAdapters}
					activeAdapters={activeAdapters.length}
					monitoringCount={monitoringCount}
					totalStats={totalStats}
					lifetimeStats={lifetimeStats}
				/>
			</Box>

			<AdapterList adapters={adapters} />
		</Box>
	);
};

export default TotalOverview;
