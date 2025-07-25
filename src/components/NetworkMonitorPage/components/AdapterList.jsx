import React from 'react';
import { 
	Card, 
	CardContent, 
	Typography, 
	List, 
	ListItem, 
	ListItemText, 
	Box, 
	Divider 
} from '@mui/material';
import { Computer, Wifi, WifiOff } from '@mui/icons-material';
import { getStatusChip } from '../utils/adapterHelpers';

const AdapterList = ({ adapters }) => {
	return (
		<Card>
			<CardContent>
				<Typography variant="h6" gutterBottom>
					Adapter Summary
				</Typography>
				<List>
					{adapters.map((adapter, index) => (
						<React.Fragment key={adapter.name}>
							<ListItem>
								<Box sx={{ mr: 2 }}>
									{adapter.is_loopback ? <Computer /> : 
									 adapter.is_up ? <Wifi color="primary" /> : <WifiOff color="disabled" />}
								</Box>
								<ListItemText
									primary={
										<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
											<Typography variant="subtitle1">
												{adapter.description || adapter.name}
											</Typography>
											{getStatusChip(adapter)}
										</Box>
									}
									secondary={
										adapter.addresses.length > 0 ? 
										`${adapter.addresses.length} address${adapter.addresses.length > 1 ? 'es' : ''}` :
										'No addresses'
									}
								/>
							</ListItem>
							{index < adapters.length - 1 && <Divider />}
						</React.Fragment>
					))}
				</List>
			</CardContent>
		</Card>
	);
};

export default AdapterList;
