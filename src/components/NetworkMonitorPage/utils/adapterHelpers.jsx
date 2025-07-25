import React from 'react';
import {
	Wifi,
	WifiOff,
	Computer,
	CheckCircle,
	Error as ErrorIcon
} from '@mui/icons-material';
import { Chip } from '@mui/material';

export const getAdapterIcon = (adapter) => {
	if (adapter.is_loopback) {
		return <Computer />;
	}
	if (adapter.is_up) {
		return <Wifi color="primary" />;
	}
	return <WifiOff color="disabled" />;
};

export const getStatusChip = (adapter) => {
	if (adapter.is_loopback) {
		return <Chip label="Loopback" size="small" variant="outlined" />;
	}
	if (adapter.is_up) {
		return <Chip 
			label="Active" 
			size="small" 
			color="success" 
			icon={<CheckCircle />} 
		/>;
	}
	return <Chip 
		label="Inactive" 
		size="small" 
		color="error" 
		icon={<ErrorIcon />} 
	/>;
};
