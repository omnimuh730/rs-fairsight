import React from 'react';
import {
	Card,
	CardContent,
	Typography,
	TableContainer,
	Table,
	TableHead,
	TableRow,
	TableCell,
	TableBody,
	Box
} from '@mui/material';
import { formatBytes, getCountryFlag } from '../utils/formatters';

const NetworkHostsTable = ({ hosts }) => {
	return (
		<Card>
			<CardContent>
				<Typography variant="h6" gutterBottom>
					Network Hosts
				</Typography>
				<TableContainer>
					<Table size="small">
						<TableHead>
							<TableRow>
								<TableCell>Host</TableCell>
								<TableCell>Country</TableCell>
								<TableCell align="right">Incoming</TableCell>
								<TableCell align="right">Outgoing</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{hosts.slice(0, 10).map((host) => (
								<TableRow key={host.ip}>
									<TableCell>
										<Typography variant="body2" sx={{ fontFamily: 'monospace' }}>
											{host.ip}
										</Typography>
										{host.hostname && (
											<Typography variant="caption" color="text.secondary">
												{host.hostname}
											</Typography>
										)}
									</TableCell>
									<TableCell>
										<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
											<span>{getCountryFlag(host.country_code)}</span>
											<Typography variant="body2">
												{host.country || 'Unknown'}
											</Typography>
										</Box>
									</TableCell>
									<TableCell align="right">
										<Typography variant="body2" color="primary">
											{formatBytes(host.incoming_bytes)}
										</Typography>
									</TableCell>
									<TableCell align="right">
										<Typography variant="body2" color="secondary">
											{formatBytes(host.outgoing_bytes)}
										</Typography>
									</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</TableContainer>
			</CardContent>
		</Card>
	);
};

export default NetworkHostsTable;
