import React from 'react';
import { Typography, Box, Button, Alert, Chip } from '@mui/material';
import { Computer, Wifi, WifiOff, PlayArrow, CheckCircle } from '@mui/icons-material';
import { getStatusChip } from '../utils/adapterHelpers';
import AdapterInfo from './AdapterInfo';
import MonitoringInterface from './MonitoringInterface';

const AdapterDetails = ({ 
	adapter, 
	isMonitoring, 
	onStartMonitoring,
	stats 
}) => {
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
					{getStatusChip(adapter)}
					<Box sx={{ ml: 'auto', display: 'flex', alignItems: 'center', gap: 1 }}>
						{isMonitoring ? (
							<>
								<Chip 
									icon={<CheckCircle />} 
									label="Monitoring Active" 
									color="success" 
									variant="outlined" 
									size="small"
								/>
							</>
						) : (
							<>
								<Chip 
									label={adapter.is_up ? "Auto-Starting..." : "Inactive"} 
									color={adapter.is_up ? "warning" : "default"}
									variant="outlined" 
									size="small"
								/>
								<Button
									variant="outlined"
									color="primary"
									startIcon={<PlayArrow />}
									onClick={() => onStartMonitoring(adapter.name)}
									disabled={true}  // Always disabled - monitoring auto-starts
									sx={{ opacity: 0.6 }}
								>
									Auto-Started
								</Button>
							</>
						)}
					</Box>
				</Box>
			</Typography>

			{!adapter.is_up && (
				<Alert severity="warning" sx={{ mb: 2 }}>
					This adapter is currently inactive. Monitoring is not available for inactive adapters.
				</Alert>
			)}

			{!isMonitoring ? (
				<AdapterInfo adapter={adapter} />
			) : (
				<MonitoringInterface adapter={adapter} stats={stats} />
			)}
		</Box>
	);
};

export default AdapterDetails;
