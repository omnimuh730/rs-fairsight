import React from 'react';
import { Paper, Typography } from '@mui/material';
import { NetworkCheck } from '@mui/icons-material';

/**
 * Header component for the Weekly Network Activity page
 */
const PageHeader = () => {
	return (
		<Paper elevation={0} sx={{ 
			mb: 4, 
			p: { xs: 3, md: 4 },
			background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
			color: 'white',
			borderRadius: 3
		}}>
			<Typography variant="h4" gutterBottom sx={{ 
				display: 'flex', 
				alignItems: 'center',
				fontSize: { xs: '1.75rem', md: '2.125rem' },
				fontWeight: 700,
				mb: 2
			}}>
				<NetworkCheck sx={{ mr: 2, fontSize: { xs: '2rem', md: '2.5rem' } }} />
				Weekly Network Activity
			</Typography>
			<Typography variant="h6" sx={{ 
				opacity: 0.95,
				fontSize: { xs: '1rem', md: '1.25rem' },
				fontWeight: 400
			}}>
				Comprehensive analysis of your network usage patterns and traffic statistics
			</Typography>
		</Paper>
	);
};

export default PageHeader;
