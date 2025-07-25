import React from 'react';
import {
	Card,
	CardContent,
	Typography,
	List,
	ListItem,
	ListItemText,
	Box,
	LinearProgress
} from '@mui/material';
import { formatBytes } from '../utils/formatters';

const ServicesList = ({ services }) => {
	const maxBytes = Math.max(...services.map(s => s.bytes));

	return (
		<Card>
			<CardContent>
				<Typography variant="h6" gutterBottom>
					Services
				</Typography>
				<List dense>
					{services.slice(0, 10).map((service) => (
						<ListItem key={`${service.protocol}:${service.port}`} disablePadding>
							<ListItemText
								primary={
									<Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
										<Typography variant="body2">
											{service.service_name || `${service.protocol}:${service.port}`}
										</Typography>
										<Typography variant="body2" color="text.secondary">
											{formatBytes(service.bytes)}
										</Typography>
									</Box>
								}
								secondary={
									<LinearProgress
										variant="determinate"
										value={(service.bytes / maxBytes) * 100}
										sx={{ mt: 0.5 }}
									/>
								}
							/>
						</ListItem>
					))}
				</List>
			</CardContent>
		</Card>
	);
};

export default ServicesList;
