import React from 'react';
import { Typography, Box, Button, Alert, Chip } from '@mui/material';
import { Computer, Wifi, WifiOff, CheckCircle } from '@mui/icons-material';
import { getStatusChip } from '../utils/adapterHelpers';
import AdapterInfo from './AdapterInfo';
import MonitoringInterface from './MonitoringInterface';

const AdapterDetails = ({ 
	adapter, 
	isMonitoring,
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
						{isMonitoring && adapter.is_up && (
							<Chip 
								icon={<CheckCircle />} 
								label="Auto-Monitoring Active" 
								color="success" 
								variant="filled" 
								size="small"
							/>
						)}
						{!adapter.is_up && (
							<Chip 
								label="Inactive" 
								color="default" 
								variant="outlined" 
								size="small"
							/>
						)}
					</Box>
				</Box>
			</Typography>

			{!adapter.is_up && (
				<Alert severity="warning" sx={{ mb: 2 }}>
					This adapter is currently inactive. Auto-monitoring is not available for inactive adapters.
				</Alert>
			)}

			{!isMonitoring || !adapter.is_up ? (
				<AdapterInfo adapter={adapter} />
			) : (
				<MonitoringInterface adapter={adapter} stats={stats} />
			)}
		</Box>
	);
};

export default AdapterDetails;
