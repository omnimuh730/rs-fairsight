import React from 'react';
import { Grid, Card, CardContent, Typography, Box, CircularProgress } from '@mui/material';
import { formatBytes, formatDuration } from '../utils/formatters';
import TrafficChart from './TrafficChart';
import NetworkHostsTable from './NetworkHostsTable';
import ServicesList from './ServicesList';

const MonitoringInterface = ({ adapter, stats, todaySummary }) => {
	const displayStats = todaySummary || stats;

    if (!displayStats) {
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
										↓ {formatBytes(displayStats.total_incoming_bytes)}
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
										↑ {formatBytes(displayStats.total_outgoing_bytes)}
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
										{displayStats.unique_hosts}
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
										                              {displayStats.sessions?.length || 0}
									</Typography>
									<Typography variant="body2" color="text.secondary">
										Sessions
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
							<TrafficChart data={displayStats.traffic_rate || []} />
						</CardContent>
					</Card>
				</Grid>
			</Grid>

			{/* Network Hosts and Services */}
			< Grid item xs={12} >
				<NetworkHostsTable hosts={displayStats.network_hosts || []} />
			</Grid >

			<Grid item xs={12} sx={{ mt: 2 }}>
				<ServicesList services={displayStats.services || []} />
			</Grid>

		</Box >
	);
};

export default MonitoringInterface;
