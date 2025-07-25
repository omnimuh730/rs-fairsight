import React from 'react';
import { Grid, Card, CardContent, Typography, Box, CircularProgress } from '@mui/material';
import { formatBytes, formatDuration } from '../utils/formatters';
import TrafficChart from './TrafficChart';
import NetworkHostsTable from './NetworkHostsTable';
import ServicesList from './ServicesList';

const MonitoringInterface = ({ adapter, stats }) => {
	if (!stats) {
		return (
			<Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: 200 }}>
				<CircularProgress />
				<Typography sx={{ ml: 2 }}>Loading monitoring data...</Typography>
			</Box>
		);
	}

	return (
		<Box sx={{ flexGrow: 1 }}>
			<Grid container spacing={2}>
				<Grid size={{ md: 12, lg: 8 }}>
					<Grid container spacing={2} sx={{ height: '100%' }}>
						<Grid size={{ md: 12, lg: 6 }}>
							<Card sx={{ height: '100%' }}>
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
						<Grid size={{ md: 12, lg: 6 }}>
							<Card sx={{ height: '100%' }}>
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
						<Grid size={{ md: 12, lg: 6 }}>
							<Card sx={{ height: '100%' }}>
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
						<Grid size={{ md: 12, lg: 6 }}>
							<Card sx={{ height: '100%' }}>
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
				<Grid size={{ md: 12, lg: 4 }}>
					<Card sx={{ height: '100%' }}>
						<CardContent>
							<Typography variant="h6" gutterBottom>
								Traffic Rate
							</Typography>
							<TrafficChart data={stats.traffic_rate} />
						</CardContent>
					</Card>
				</Grid>
			</Grid>

			{/* Network Hosts and Services */}
			< Grid item xs={12} >
				<NetworkHostsTable hosts={stats.network_hosts} />
			</Grid >

			<Grid item xs={12} sx={{ mt: 2 }}>
				<ServicesList services={stats.services} />
			</Grid>

		</Box >
	);
};

export default MonitoringInterface;
