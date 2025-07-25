import React from 'react';
import { Card, CardContent, Typography, Box, Grid, Paper } from '@mui/material';
import {
	TrendingUp,
	CloudDownload,
	CloudUpload,
	Schedule,
	Computer,
	NetworkCheck
} from '@mui/icons-material';
import StatCard from './StatCard';
import { formatBytes, formatDuration } from '../utils/formatters';
import { styled } from '@mui/material/styles';

/**
 * Summary statistics section component
 */

const Item = styled(Paper)(({ theme }) => ({
	backgroundColor: '#fff',
	...theme.typography.body2,
	padding: theme.spacing(1),
	textAlign: 'center',
	color: (theme.vars ?? theme).palette.text.secondary,
	...theme.applyStyles('dark', {
		backgroundColor: '#1A2027',
	}),
}));

const SummaryStatistics = ({ totalStats }) => {
	return (
		<Card sx={{
			mb: 4,
			boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
			borderRadius: 3,
			border: '1px solid rgba(255,255,255,0.2)'
		}}>
			<CardContent sx={{ p: { xs: 3, md: 4 } }}>
				<Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
					<TrendingUp sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
					<Typography variant="h5" sx={{ fontWeight: 600, color: 'text.primary' }}>
						Summary Statistics
					</Typography>
				</Box>

				<Box sx={{ flexGrow: 1 }}>
					<Grid container spacing={2}>
						<Grid size={6}>
							<Item>
								<StatCard
									icon={CloudDownload}
									title="Incoming"
									value={formatBytes(totalStats.totalIncoming)}
									color="primary"
									colorValue="primary.main"
								/>
							</Item>
						</Grid>

						<Grid size={6}>
							<Item>
								<StatCard
									icon={CloudUpload}
									title="Outgoing"
									value={formatBytes(totalStats.totalOutgoing)}
									color="secondary"
									colorValue="secondary.main"
								/>
							</Item>
						</Grid>

						<Grid size={{md: 6, lg: 3}}>
							<Item>
								<StatCard
									icon={Schedule}
									title="Duration"
									value={formatDuration(totalStats.totalDuration)}
									color="info"
									colorValue="info.main"
								/>
							</Item>
						</Grid>

						<Grid size={{md: 6, lg: 3}}>
							<Item>
								<StatCard
									icon={Computer}
									title="Hosts"
									value={totalStats.uniqueHosts}
									color="success"
									colorValue="success.main"
								/>
							</Item>
						</Grid>
						<Grid size={{md: 6, lg: 3}}>
							<Item>
								<StatCard
									icon={TrendingUp}
									title="Services"
									value={totalStats.uniqueServices}
									color="warning"
									colorValue="warning.main"
								/>
							</Item>
						</Grid>
						<Grid size={{md: 6, lg: 3}}>
							<Item>
								<StatCard
									icon={NetworkCheck}
									title="Sessions"
									value={totalStats.totalSessions}
									color="purple"
									colorValue="#9c27b0"
								/>
							</Item>
						</Grid>
					</Grid>
				</Box>
			</CardContent>
		</Card >
	);
};

export default SummaryStatistics;
