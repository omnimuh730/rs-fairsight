import React from 'react';
import { Box, Typography } from '@mui/material';

const TrafficChart = ({ data }) => {
	if (!data || data.length === 0) {
		return (
			<Box sx={{ height: 200, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
				<Typography color="text.secondary">No data available</Typography>
			</Box>
		);
	}

	const maxBytes = Math.max(...data.map(d => Math.max(d.incoming_bytes, d.outgoing_bytes)));
	
	return (
		<Box sx={{ height: 200, display: 'flex', alignItems: 'end', gap: 1, overflow: 'hidden' }}>
			{data.slice(-60).map((point, index) => (
				<Box key={index} sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', minWidth: 3 }}>
					<Box
						sx={{
							width: 3,
							height: Math.max(1, (point.incoming_bytes / maxBytes) * 100),
							backgroundColor: 'primary.main',
							mb: 0.25
						}}
					/>
					<Box
						sx={{
							width: 3,
							height: Math.max(1, (point.outgoing_bytes / maxBytes) * 100),
							backgroundColor: 'secondary.main'
						}}
					/>
				</Box>
			))}
		</Box>
	);
};

export default TrafficChart;
