import React from 'react';
import {
	Card,
	CardContent,
	Typography,
	Box,
	Grid,
	List,
	ListItem,
	ListItemText,
	Chip
} from '@mui/material';
import {
	Computer,
	CloudDownload,
	CloudUpload,
	TrendingUp
} from '@mui/icons-material';
import dayjs from 'dayjs';
import { formatBytes, formatDuration } from '../utils/formatters';

/**
 * Daily Details section showing detailed breakdown by day
 */
const DailyDetailsSection = ({ networkData }) => {
	if (!networkData || networkData.length === 0) {
		return null;
	}

	return (
		<Card sx={{ 
			boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
			borderRadius: 3,
			border: '1px solid rgba(255,255,255,0.2)'
		}}>
			<CardContent sx={{ p: { xs: 3, md: 4 } }}>
				<Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
					<Computer sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
					<Typography variant="h5" sx={{ fontWeight: 600, color: 'text.primary' }}>
						Daily Details
					</Typography>
				</Box>
				<Box sx={{ 
					maxHeight: { xs: 450, md: 550 }, 
					overflow: 'auto',
					bgcolor: 'grey.50',
					borderRadius: 2,
					p: 1,
					'&::-webkit-scrollbar': {
						width: '8px',
					},
					'&::-webkit-scrollbar-track': {
						background: '#f1f1f1',
						borderRadius: '4px',
					},
					'&::-webkit-scrollbar-thumb': {
						background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
						borderRadius: '4px',
					},
					'&::-webkit-scrollbar-thumb:hover': {
						background: 'linear-gradient(45deg, #1976D2 30%, #1CB5E0 90%)',
					},
				}}>
					<List sx={{ p: 0 }}>
						{networkData.map((day, index) => (
							<React.Fragment key={day.date}>
								<Card sx={{
									mb: 2,
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-2px)',
										boxShadow: '0 8px 25px rgba(0,0,0,0.1)'
									}
								}}>
									<ListItem sx={{ 
										py: 3,
										px: 3
									}}>
										<ListItemText
											primary={
												<Box sx={{ 
													display: 'flex', 
													justifyContent: 'space-between', 
													alignItems: { xs: 'flex-start', sm: 'center' },
													flexDirection: { xs: 'column', sm: 'row' },
													gap: { xs: 2, sm: 0 },
													mb: 2
												}}>
													<Typography variant="h6" fontWeight="bold" sx={{
														fontSize: { xs: '1.1rem', md: '1.25rem' },
														color: 'primary.main'
													}}>
														{dayjs(day.date).format('dddd, MMMM D, YYYY')}
													</Typography>
													<Box sx={{ 
														display: 'flex', 
														gap: 1.5, 
														flexWrap: 'wrap',
														alignItems: 'center'
													}}>
														<Chip 
															label={`${day.sessions.length} session${day.sessions.length !== 1 ? 's' : ''}`} 
															size="medium" 
															color="primary"
															variant="filled"
															sx={{ 
																fontWeight: 600,
																borderRadius: 3
															}}
														/>
														<Chip 
															label={formatBytes(day.total_incoming_bytes + day.total_outgoing_bytes)} 
															size="medium" 
															color="secondary"
															variant="filled"
															sx={{ 
																fontWeight: 600,
																borderRadius: 3
															}}
														/>
														{day.total_duration > 0 && (
															<Chip 
																label={formatDuration(day.total_duration)} 
																size="medium" 
																color="info"
																variant="filled"
																sx={{ 
																	fontWeight: 600,
																	borderRadius: 3
																}}
															/>
														)}
													</Box>
												</Box>
											}
											secondary={
												<Box sx={{ mt: 2 }}>
													<Grid container spacing={2}>
														<Grid item xs={6} sm={3}>
															<Box sx={{ 
																display: 'flex', 
																alignItems: 'center', 
																gap: 1.5,
																p: 2,
																bgcolor: 'primary.50',
																borderRadius: 2,
																border: '1px solid',
																borderColor: 'primary.200'
															}}>
																<CloudDownload sx={{ fontSize: 20, color: 'primary.main' }} />
																<Box>
																	<Typography variant="caption" color="text.secondary" sx={{
																		fontSize: '0.75rem',
																		fontWeight: 500
																	}}>
																		Downloaded
																	</Typography>
																	<Typography variant="body2" fontWeight="bold" sx={{
																		fontSize: { xs: '0.875rem', md: '1rem' },
																		color: 'primary.main'
																	}}>
																		{formatBytes(day.total_incoming_bytes)}
																	</Typography>
																</Box>
															</Box>
														</Grid>
														<Grid item xs={6} sm={3}>
															<Box sx={{ 
																display: 'flex', 
																alignItems: 'center', 
																gap: 1.5,
																p: 2,
																bgcolor: 'secondary.50',
																borderRadius: 2,
																border: '1px solid',
																borderColor: 'secondary.200'
															}}>
																<CloudUpload sx={{ fontSize: 20, color: 'secondary.main' }} />
																<Box>
																	<Typography variant="caption" color="text.secondary" sx={{
																		fontSize: '0.75rem',
																		fontWeight: 500
																	}}>
																		Uploaded
																	</Typography>
																	<Typography variant="body2" fontWeight="bold" sx={{
																		fontSize: { xs: '0.875rem', md: '1rem' },
																		color: 'secondary.main'
																	}}>
																		{formatBytes(day.total_outgoing_bytes)}
																	</Typography>
																</Box>
															</Box>
														</Grid>
														<Grid item xs={6} sm={3}>
															<Box sx={{ 
																display: 'flex', 
																alignItems: 'center', 
																gap: 1.5,
																p: 2,
																bgcolor: 'success.50',
																borderRadius: 2,
																border: '1px solid',
																borderColor: 'success.200'
															}}>
																<Computer sx={{ fontSize: 20, color: 'success.main' }} />
																<Box>
																	<Typography variant="caption" color="text.secondary" sx={{
																		fontSize: '0.75rem',
																		fontWeight: 500
																	}}>
																		Hosts
																	</Typography>
																	<Typography variant="body2" fontWeight="bold" sx={{
																		fontSize: { xs: '0.875rem', md: '1rem' },
																		color: 'success.main'
																	}}>
																		{day.unique_hosts}
																	</Typography>
																</Box>
															</Box>
														</Grid>
														<Grid item xs={6} sm={3}>
															<Box sx={{ 
																display: 'flex', 
																alignItems: 'center', 
																gap: 1.5,
																p: 2,
																bgcolor: 'warning.50',
																borderRadius: 2,
																border: '1px solid',
																borderColor: 'warning.200'
															}}>
																<TrendingUp sx={{ fontSize: 20, color: 'warning.main' }} />
																<Box>
																	<Typography variant="caption" color="text.secondary" sx={{
																		fontSize: '0.75rem',
																		fontWeight: 500
																	}}>
																		Services
																	</Typography>
																	<Typography variant="body2" fontWeight="bold" sx={{
																		fontSize: { xs: '0.875rem', md: '1rem' },
																		color: 'warning.main'
																	}}>
																		{day.unique_services}
																	</Typography>
																</Box>
															</Box>
														</Grid>
													</Grid>
												</Box>
											}
										/>
									</ListItem>
								</Card>
							</React.Fragment>
						))}
					</List>
				</Box>
			</CardContent>
		</Card>
	);
};

export default DailyDetailsSection;
