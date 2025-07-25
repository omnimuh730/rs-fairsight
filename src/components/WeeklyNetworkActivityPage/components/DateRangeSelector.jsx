import React from 'react';
import {
	Card,
	CardContent,
	Typography,
	Box,
	Button,
	Grid,
	CircularProgress
} from '@mui/material';
import { DatePicker } from '@mui/x-date-pickers';
import { Schedule, Refresh } from '@mui/icons-material';

/**
 * Date selection component for filtering network data
 */
const DateRangeSelector = ({ 
	startDate, 
	endDate, 
	onStartDateChange, 
	onEndDateChange, 
	onFetchData, 
	loading, 
	dataCount 
}) => {
	return (
		<Card sx={{ 
			mb: 4, 
			boxShadow: '0 8px 32px rgba(0,0,0,0.1)', 
			borderRadius: 3,
			border: '1px solid rgba(255,255,255,0.2)'
		}}>
			<CardContent sx={{ p: { xs: 3, md: 4 } }}>
				<Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
					<Schedule sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
					<Typography variant="h5" sx={{ 
						fontWeight: 600,
						color: 'text.primary'
					}}>
						Select Date Range
					</Typography>
				</Box>
				<Grid container spacing={3} alignItems="end">
					<Grid item xs={12} sm={6} md={3}>
						<DatePicker
							label="Start Date"
							value={startDate}
							onChange={onStartDateChange}
							slotProps={{ 
								textField: { 
									fullWidth: true,
									size: 'medium',
									variant: 'outlined'
								} 
							}}
						/>
					</Grid>
					<Grid item xs={12} sm={6} md={3}>
						<DatePicker
							label="End Date"
							value={endDate}
							onChange={onEndDateChange}
							slotProps={{ 
								textField: { 
									fullWidth: true,
									size: 'medium',
									variant: 'outlined'
								} 
							}}
						/>
					</Grid>
					<Grid item xs={12} sm={8} md={4}>
						<Button
							variant="contained"
							onClick={onFetchData}
							disabled={loading}
							startIcon={loading ? <CircularProgress size={20} color="inherit" /> : <Refresh />}
							fullWidth
							size="large"
							sx={{ 
								py: 2,
								borderRadius: 2,
								textTransform: 'none',
								fontSize: '1rem',
								fontWeight: 600,
								background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
								'&:hover': {
									background: 'linear-gradient(45deg, #1976D2 30%, #1CB5E0 90%)',
								}
							}}
						>
							{loading ? 'Loading...' : 'Fetch Data'}
						</Button>
					</Grid>
					<Grid item xs={12} sm={4} md={2}>
						<Box sx={{ 
							textAlign: 'center',
							p: 2,
							bgcolor: 'action.hover',
							borderRadius: 2,
							border: '1px solid',
							borderColor: 'divider'
						}}>
							<Typography variant="body2" color="text.secondary" fontWeight={500}>
								{dataCount > 0 ? `${dataCount} days loaded` : 'No data'}
							</Typography>
						</Box>
					</Grid>
				</Grid>
			</CardContent>
		</Card>
	);
};

export default DateRangeSelector;
