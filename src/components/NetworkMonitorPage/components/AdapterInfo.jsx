import React from 'react';
import { 
	Grid, 
	Card, 
	CardContent, 
	Typography, 
	Box, 
	List, 
	ListItem, 
	ListItemText,
	Chip,
	Alert
} from '@mui/material';

const AdapterInfo = ({ adapter }) => {
	return (
		<Grid container spacing={3}>
			<Grid item xs={12} md={6}>
				<Card>
					<CardContent>
						<Typography variant="h6" gutterBottom>
							Adapter Information
						</Typography>
						
						<Box sx={{ mb: 2 }}>
							<Typography variant="body2" color="text.secondary">
								Name
							</Typography>
							<Typography variant="body1" sx={{ fontFamily: 'monospace', wordBreak: 'break-all' }}>
								{adapter.name}
							</Typography>
						</Box>

						{adapter.description && (
							<Box sx={{ mb: 2 }}>
								<Typography variant="body2" color="text.secondary">
									Description
								</Typography>
								<Typography variant="body1">
									{adapter.description}
								</Typography>
							</Box>
						)}

						<Box sx={{ mb: 2 }}>
							<Typography variant="body2" color="text.secondary">
								Status
							</Typography>
							<Box sx={{ display: 'flex', gap: 1, mt: 1 }}>
								<Chip 
									label={adapter.is_up ? "Active" : "Inactive"} 
									color={adapter.is_up ? "success" : "error"}
									size="small"
								/>
								{adapter.is_loopback && (
									<Chip label="Loopback" variant="outlined" size="small" />
								)}
							</Box>
						</Box>
					</CardContent>
				</Card>
			</Grid>

			<Grid item xs={12} md={6}>
				<Card>
					<CardContent>
						<Typography variant="h6" gutterBottom>
							Network Addresses
						</Typography>
						
						{adapter.addresses.length === 0 ? (
							<Typography color="text.secondary">
								No addresses assigned
							</Typography>
						) : (
							<List dense>
								{adapter.addresses.map((address, idx) => (
									<ListItem key={idx} disablePadding>
										<ListItemText
											primary={
												<Typography 
													variant="body2" 
													sx={{ fontFamily: 'monospace' }}
												>
													{address}
												</Typography>
											}
										/>
									</ListItem>
								))}
							</List>
						)}
					</CardContent>
				</Card>
			</Grid>

			<Grid item xs={12}>
				<Alert severity="info">
					Click "Start Monitoring" to begin capturing network traffic for this adapter.
					You'll see real-time statistics, network hosts, and service information.
				</Alert>
			</Grid>
		</Grid>
	);
};

export default AdapterInfo;
