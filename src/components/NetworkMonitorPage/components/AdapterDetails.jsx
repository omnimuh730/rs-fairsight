import React from 'react';
import { Typography, Box, Button, Alert } from '@mui/material';
import { Computer, Wifi, WifiOff, Stop, PlayArrow } from '@mui/icons-material';
import { getStatusChip } from '../utils/adapterHelpers';
import AdapterInfo from './AdapterInfo';
import MonitoringInterface from './MonitoringInterface';

const AdapterDetails = ({ 
	adapter, 
	onSelect, 
	isMonitoring, 
	onStartMonitoring, 
	onStopMonitoring, 
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
					<Box sx={{ ml: 'auto' }}>
						{isMonitoring ? (
							<Button
								variant="contained"
								color="error"
								startIcon={<Stop />}
								onClick={() => onStopMonitoring(adapter.name)}
							>
								Stop Monitoring
							</Button>
						) : (
							<Button
								variant="contained"
								color="primary"
								startIcon={<PlayArrow />}
								onClick={() => onStartMonitoring(adapter.name)}
								disabled={!adapter.is_up}
							>
								Start Monitoring
							</Button>
						)}
					</Box>
				</Box>
			</Typography>

			{!isMonitoring ? (
				<AdapterInfo adapter={adapter} />
			) : (
				<MonitoringInterface adapter={adapter} stats={stats} />
			)}
		</Box>
	);
};

export default AdapterDetails;
