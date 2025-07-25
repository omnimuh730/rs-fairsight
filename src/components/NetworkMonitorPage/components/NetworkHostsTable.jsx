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
	Box,
	Chip,
	Tooltip
} from '@mui/material';
import { 
	Language,
	Public,
	Computer
} from '@mui/icons-material';
import { 
	formatBytes, 
	getCountryFlag, 
	formatDomain, 
	formatASN, 
	getHostTypeIcon,
	getCountryName 
} from '../utils/formatters';

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
								<TableCell>Domain</TableCell>
								<TableCell>Country</TableCell>
								<TableCell>ASN</TableCell>
								<TableCell align="right">Incoming</TableCell>
								<TableCell align="right">Outgoing</TableCell>
								<TableCell align="right">Total</TableCell>
							</TableRow>
						</TableHead>
						<TableBody>
							{hosts.slice(0, 10).map((host) => (
								<TableRow key={host.ip}>
									<TableCell>
										<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
											<span style={{ fontSize: '16px' }}>
												{getHostTypeIcon(host.ip, host.hostname, host.domain)}
											</span>
											<Box>
												<Typography variant="body2" sx={{ fontFamily: 'monospace', fontWeight: 'bold' }}>
													{host.ip}
												</Typography>
												{host.hostname && host.hostname !== host.ip && (
													<Tooltip title="Resolved hostname">
														<Typography variant="caption" color="text.secondary" sx={{ display: 'block' }}>
															{host.hostname}
														</Typography>
													</Tooltip>
												)}
											</Box>
										</Box>
									</TableCell>
									<TableCell>
										{formatDomain(host.hostname, host.domain) ? (
											<Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
												<Language sx={{ fontSize: 14, color: 'primary.main' }} />
												<Typography variant="body2" color="primary">
													{formatDomain(host.hostname, host.domain)}
												</Typography>
											</Box>
										) : (
											<Typography variant="caption" color="text.disabled">
												No domain
											</Typography>
										)}
									</TableCell>
									<TableCell>
										{host.country && host.country_code ? (
											<Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
												<span style={{ fontSize: '16px' }}>{getCountryFlag(host.country_code)}</span>
												<Typography variant="body2">
													{getCountryName(host.country_code) || host.country}
												</Typography>
											</Box>
										) : (
											<Typography variant="caption" color="text.disabled">
												Unknown
											</Typography>
										)}
									</TableCell>
									<TableCell>
										{formatASN(host.asn) ? (
											<Chip 
												label={formatASN(host.asn)} 
												size="small" 
												variant="outlined"
												icon={<Public sx={{ fontSize: 14 }} />}
											/>
										) : (
											<Typography variant="caption" color="text.disabled">
												No ASN
											</Typography>
										)}
									</TableCell>
									<TableCell align="right">
										<Typography variant="body2" color="primary" sx={{ fontWeight: 'medium' }}>
											{formatBytes(host.incoming_bytes)}
										</Typography>
										<Typography variant="caption" color="text.secondary" sx={{ display: 'block' }}>
											{host.incoming_packets} pkts
										</Typography>
									</TableCell>
									<TableCell align="right">
										<Typography variant="body2" color="secondary" sx={{ fontWeight: 'medium' }}>
											{formatBytes(host.outgoing_bytes)}
										</Typography>
										<Typography variant="caption" color="text.secondary" sx={{ display: 'block' }}>
											{host.outgoing_packets} pkts
										</Typography>
									</TableCell>
									<TableCell align="right">
										<Typography variant="body2" sx={{ fontWeight: 'bold' }}>
											{formatBytes(host.incoming_bytes + host.outgoing_bytes)}
										</Typography>
										<Typography variant="caption" color="text.secondary" sx={{ display: 'block' }}>
											{host.incoming_packets + host.outgoing_packets} pkts
										</Typography>
									</TableCell>
								</TableRow>
							))}
						</TableBody>
					</Table>
				</TableContainer>
				{hosts.length === 0 && (
					<Box sx={{ textAlign: 'center', py: 3 }}>
						<Typography variant="body2" color="text.secondary">
							No network hosts detected yet. Start monitoring to see traffic data.
						</Typography>
					</Box>
				)}
			</CardContent>
		</Card>
	);
};

export default NetworkHostsTable;
