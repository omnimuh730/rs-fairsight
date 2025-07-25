import React from 'react';
import { Card, CardContent, Typography, Box } from '@mui/material';

/**
 * Individual statistic card component
 */
const StatCard = ({ 
	icon: Icon, 
	title, 
	value, 
	color = 'primary',
	colorValue = 'primary.main'
}) => {
	return (
		<Card sx={{ 
			height: '100%', 
			boxShadow: `0 4px 20px rgba(${color === 'primary' ? '33, 150, 243' : color === 'secondary' ? '255, 152, 0' : color === 'info' ? '33, 150, 243' : color === 'success' ? '76, 175, 80' : color === 'warning' ? '255, 152, 0' : '156, 39, 176'}, 0.15)`,
			borderRadius: 2,
			border: `2px solid rgba(${color === 'primary' ? '33, 150, 243' : color === 'secondary' ? '255, 152, 0' : color === 'info' ? '33, 150, 243' : color === 'success' ? '76, 175, 80' : color === 'warning' ? '255, 152, 0' : '156, 39, 176'}, 0.1)`,
			transition: 'all 0.3s ease',
			'&:hover': {
				transform: 'translateY(-4px)',
				boxShadow: `0 8px 25px rgba(${color === 'primary' ? '33, 150, 243' : color === 'secondary' ? '255, 152, 0' : color === 'info' ? '33, 150, 243' : color === 'success' ? '76, 175, 80' : color === 'warning' ? '255, 152, 0' : '156, 39, 176'}, 0.25)`,
				borderColor: colorValue
			}
		}}>
			<CardContent sx={{ p: 3, textAlign: 'center' }}>
				<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
					<Icon sx={{ 
						fontSize: 48, 
						color: colorValue, 
						mb: 1.5 
					}} />
					<Typography variant="h6" sx={{ 
						fontSize: '1rem',
						fontWeight: 700,
						textTransform: 'uppercase',
						letterSpacing: 1,
						color: colorValue
					}}>
						{title}
					</Typography>
				</Box>
				<Typography variant="h4" sx={{ 
					fontSize: { sm: '1.3rem', md: '1.15rem' },
					fontWeight: 800,
					color: 'text.primary'
				}}>
					{value}
				</Typography>
			</CardContent>
		</Card>
	);
};

export default StatCard;
