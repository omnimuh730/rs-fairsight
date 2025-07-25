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
				<NetworkHostsTable hosts={stats.network_hosts} />
			</Grid>

			<Grid item xs={12}>
				<ServicesList services={stats.services} />
			</Grid>
		</Grid>
	);
};

export default MonitoringInterface;
