import React, { useState, useEffect } from 'react';
import { 
	Paper, 
	Typography, 
	Box, 
	Button,
	Alert,
	Chip,
	CircularProgress 
} from '@mui/material';
import { 
	Refresh, 
	Storage, 
	NetworkCheck,
	DataUsage,
	Warning,
	CheckCircle 
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';

const NetworkDataSync = ({ onDataUpdate }) => {
	const [currentTotals, setCurrentTotals] = useState(null);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState(null);

	const formatBytes = (bytes) => {
		if (!bytes || bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};

	const fetchCurrentTotals = async () => {
		setLoading(true);
		setError(null);
		try {
			const totals = await invoke('get_current_network_totals');
			setCurrentTotals(totals);
			if (onDataUpdate) {
				onDataUpdate(totals);
			}
		} catch (err) {
			setError(err.toString());
			console.error('Failed to fetch current network totals:', err);
		} finally {
			setLoading(false);
		}
	};

	useEffect(() => {
		fetchCurrentTotals();
		// Auto-refresh every 10 seconds
		const interval = setInterval(fetchCurrentTotals, 10000);
		return () => clearInterval(interval);
	}, []);

	if (loading && !currentTotals) {
		return (
			<Paper elevation={1} sx={{ p: 3, mb: 3 }}>
				<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
					<CircularProgress size={24} sx={{ mr: 2 }} />
					<Typography>Loading network data sync...</Typography>
				</Box>
			</Paper>
		);
	}

	if (error) {
		return (
			<Alert severity="error" sx={{ mb: 3 }}>
				<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
					<span>Failed to sync network data: {error}</span>
					<Button
						variant="outlined"
						size="small"
						startIcon={<Refresh />}
						onClick={fetchCurrentTotals}
					>
						Retry
					</Button>
				</Box>
			</Alert>
		);
	}

	if (!currentTotals) {
		return null;
	}

	const combined = currentTotals.combined_totals;
	const persistentHasData = combined.total_incoming_bytes > 0 || combined.total_outgoing_bytes > 0;
	const sessionHasData = combined.session_incoming_bytes > 0 || combined.session_outgoing_bytes > 0;
	const dataDiscrepancy = Math.abs(combined.total_incoming_bytes - combined.session_incoming_bytes) > 1024; // More than 1KB difference

	return (
		<Paper elevation={1} sx={{ p: 3, mb: 3 }}>
			<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
				<Box sx={{ display: 'flex', alignItems: 'center' }}>
					<DataUsage color="primary" sx={{ mr: 1 }} />
					<Typography variant="h6">Network Data Synchronization</Typography>
				</Box>
				<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
					{loading && <CircularProgress size={20} />}
					<Button
						variant="outlined"
						size="small"
						startIcon={<Refresh />}
						onClick={fetchCurrentTotals}
						disabled={loading}
					>
						Refresh
					</Button>
				</Box>
			</Box>

			{dataDiscrepancy && (
				<Alert severity="warning" sx={{ mb: 2 }}>
					<Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
						<Box sx={{ display: 'flex', alignItems: 'center' }}>
							<Warning sx={{ mr: 1 }} />
							Data synchronization notice: Real-time monitoring data is the authoritative source. Session data may show higher values due to incremental saves.
						</Box>
						{combined.today_sessions_count > 100 && (
							<Button
								variant="outlined"
								size="small"
								onClick={async () => {
									try {
										const result = await invoke('consolidate_today_sessions');
										console.log('Session consolidation result:', result);
										fetchCurrentTotals(); // Refresh data
									} catch (err) {
										console.error('Failed to consolidate sessions:', err);
									}
								}}
							>
								Consolidate Sessions
							</Button>
						)}
					</Box>
				</Alert>
			)}

			<Box sx={{ 
				display: 'grid', 
				gridTemplateColumns: { xs: '1fr', md: 'repeat(2, 1fr)' },
				gap: 2 
			}}>
				{/* Real-time (Today's Accumulated) Data */}
				<Box sx={{ p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
					<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
						<NetworkCheck color="success" sx={{ mr: 1, fontSize: 20 }} />
						<Typography variant="subtitle2" color="text.secondary">
							Today's Accumulated Data
						</Typography>
						<Chip 
							label="Live" 
							color="success" 
							size="small" 
							sx={{ ml: 1 }}
						/>
					</Box>
					<Typography variant="h6" color="primary">
						{formatBytes(combined.total_incoming_bytes + combined.total_outgoing_bytes)}
					</Typography>
					<Typography variant="caption" color="text.secondary">
						↓ {formatBytes(combined.total_incoming_bytes)} ↑ {formatBytes(combined.total_outgoing_bytes)}
					</Typography>
					<Typography variant="caption" display="block" color="text.secondary">
						{combined.active_adapters} active adapters
					</Typography>
				</Box>

				{/* Session Storage Data */}
				<Box sx={{ p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
					<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
						<Storage color="info" sx={{ mr: 1, fontSize: 20 }} />
						<Typography variant="subtitle2" color="text.secondary">
							Today's Saved Sessions
						</Typography>
						<Chip 
							label="Saved" 
							color="info" 
							size="small" 
							sx={{ ml: 1 }}
						/>
					</Box>
					<Typography variant="h6" color="info.main">
						{formatBytes(combined.session_incoming_bytes + combined.session_outgoing_bytes)}
					</Typography>
					<Typography variant="caption" color="text.secondary">
						↓ {formatBytes(combined.session_incoming_bytes)} ↑ {formatBytes(combined.session_outgoing_bytes)}
					</Typography>
					<Typography variant="caption" display="block" color="text.secondary">
						{combined.today_sessions_count} saved sessions
					</Typography>
				</Box>
			</Box>

			{/* Data Status */}
			<Box sx={{ mt: 2, display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
				<Box sx={{ display: 'flex', alignItems: 'center' }}>
					{persistentHasData ? (
						<CheckCircle color="success" sx={{ mr: 1, fontSize: 20 }} />
					) : (
						<Warning color="warning" sx={{ mr: 1, fontSize: 20 }} />
					)}
					<Typography variant="body2" color="text.secondary">
						{persistentHasData 
							? "Network monitoring is active and capturing data"
							: "No network activity detected yet"
						}
					</Typography>
				</Box>
				<Typography variant="caption" color="text.secondary">
					Last updated: {new Date().toLocaleTimeString()}
				</Typography>
			</Box>
		</Paper>
	);
};

export default NetworkDataSync;
