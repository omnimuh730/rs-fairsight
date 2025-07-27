import React, { useState } from 'react';
import {
	Box,
	Paper,
	Typography,
	Switch,
	FormControlLabel,
	Alert,
	Chip,
	Button,
	Tooltip
} from '@mui/material';
import {
	Storage,
	Warning,
	CheckCircle,
	Refresh,
	History,
	DataUsage
} from '@mui/icons-material';

const DataPersistenceStatus = ({ 
	lifetimeStats, 
	unexpectedShutdown, 
	onRefreshLifetimeStats,
	adapters = [] 
}) => {
	const [showLifetimeView, setShowLifetimeView] = useState(false);

	const formatBytes = (bytes) => {
		if (!bytes || bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};

	const formatDate = (timestamp) => {
		if (!timestamp) return 'Never';
		return new Date(timestamp * 1000).toLocaleString();
	};

	const getTotalLifetimeStats = () => {
		if (!lifetimeStats || Object.keys(lifetimeStats).length === 0) {
			return { incoming: 0, outgoing: 0 };
		}

		const total = Object.values(lifetimeStats).reduce((acc, stats) => ({
			incoming: acc.incoming + (stats.lifetime_incoming_bytes || 0),
			outgoing: acc.outgoing + (stats.lifetime_outgoing_bytes || 0)
		}), { incoming: 0, outgoing: 0 });

		return total;
	};

	const totalLifetime = getTotalLifetimeStats();

	return (
		<Paper elevation={1} sx={{ p: 3, mb: 3 }}>
			<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
				<Box sx={{ display: 'flex', alignItems: 'center' }}>
					<Storage color="primary" sx={{ mr: 1 }} />
					<Typography variant="h6">Data Persistence Status</Typography>
				</Box>
				<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
					<Tooltip title="Refresh lifetime statistics">
						<Button
							variant="outlined"
							size="small"
							startIcon={<Refresh />}
							onClick={onRefreshLifetimeStats}
						>
							Refresh
						</Button>
					</Tooltip>
				</Box>
			</Box>

			{unexpectedShutdown && (
				<Alert severity="warning" sx={{ mb: 2 }}>
					<Box sx={{ display: 'flex', alignItems: 'center' }}>
						<Warning sx={{ mr: 1 }} />
						Previous session ended unexpectedly. Some data from the last session may have been lost, 
						but lifetime totals have been preserved.
					</Box>
				</Alert>
			)}

			<Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
				<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
					<Box sx={{ display: 'flex', alignItems: 'center' }}>
						<CheckCircle color="success" sx={{ mr: 1, fontSize: 20 }} />
						<Typography variant="body2">
							Data persistence is active - all traffic is automatically saved
						</Typography>
					</Box>
					<Chip 
						label={unexpectedShutdown ? "Recovery Mode" : "Normal Operation"} 
						color={unexpectedShutdown ? "warning" : "success"}
						size="small"
					/>
				</Box>

				<Box sx={{ 
					display: 'grid', 
					gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
					gap: 2,
					mt: 1
				}}>
					<Box sx={{ p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
						<Typography variant="caption" color="text.secondary">
							Total Lifetime Traffic
						</Typography>
						<Typography variant="h6" color="primary">
							{formatBytes(totalLifetime.incoming + totalLifetime.outgoing)}
						</Typography>
						<Typography variant="caption" color="text.secondary">
							↓ {formatBytes(totalLifetime.incoming)} ↑ {formatBytes(totalLifetime.outgoing)}
						</Typography>
					</Box>

					<Box sx={{ p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
						<Typography variant="caption" color="text.secondary">
							Tracked Adapters
						</Typography>
						<Typography variant="h6" color="primary">
							{Object.keys(lifetimeStats || {}).length}
						</Typography>
						<Typography variant="caption" color="text.secondary">
							of {adapters.length} total
						</Typography>
					</Box>

					<Box sx={{ p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
						<Typography variant="caption" color="text.secondary">
							Data Since
						</Typography>
						<Typography variant="body2" color="primary">
							{Object.values(lifetimeStats || {}).length > 0 ? 
								formatDate(Math.min(...Object.values(lifetimeStats).map(s => s.first_recorded_time).filter(Boolean))) :
								'No data yet'
							}
						</Typography>
					</Box>
				</Box>

				{Object.keys(lifetimeStats || {}).length > 0 && (
					<Box sx={{ mt: 2 }}>
						<Typography variant="subtitle2" color="text.secondary" gutterBottom>
							Per-Adapter Lifetime Stats:
						</Typography>
						<Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
							{Object.entries(lifetimeStats).map(([adapterName, stats]) => (
								<Tooltip 
									key={adapterName}
									title={`First seen: ${formatDate(stats.first_recorded_time)} | Last update: ${formatDate(stats.last_update_time)}`}
								>
									<Chip
										icon={<DataUsage />}
										label={`${adapterName.split(' ')[0]}: ${formatBytes(stats.lifetime_incoming_bytes + stats.lifetime_outgoing_bytes)}`}
										variant="outlined"
										size="small"
									/>
								</Tooltip>
							))}
						</Box>
					</Box>
				)}
			</Box>
		</Paper>
	);
};

export default DataPersistenceStatus;
