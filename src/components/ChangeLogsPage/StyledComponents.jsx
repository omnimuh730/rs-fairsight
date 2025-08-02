import { styled } from '@mui/material/styles';
import { Box, Card, Typography, Chip } from '@mui/material';
import TimelineItem from '@mui/lab/TimelineItem';
import TimelineDot from '@mui/lab/TimelineDot';

export const StyledTimelineItem = styled(TimelineItem)(({ theme, index }) => ({
	'&::before': {
		display: 'none', // Remove the default line
	},
	'& .MuiTimelineItem-content': {
		display: 'flex',
		justifyContent: index % 2 === 0 ? 'flex-end' : 'flex-start',
		paddingLeft: index % 2 === 0 ? theme.spacing(3) : theme.spacing(1),
		paddingRight: index % 2 === 0 ? theme.spacing(1) : theme.spacing(3),
		alignItems: 'center',
	},
	'& .MuiTimelineItem-oppositeContent': {
		textAlign: index % 2 === 0 ? 'right' : 'left',
		flex: 0.3,
		paddingLeft: index % 2 === 0 ? theme.spacing(2) : 0,
		paddingRight: index % 2 === 0 ? 0 : theme.spacing(2),
		maxWidth: '180px',
		display: 'flex',
		flexDirection: 'column',
		justifyContent: 'center',
		alignItems: index % 2 === 0 ? 'flex-end' : 'flex-start',
	}
}));

export const StyledTimelineDot = styled(TimelineDot)(({ theme, versionColor }) => ({
	animation: 'bounceIn 1s ease-out',
	transition: 'all 0.3s ease',
	width: 60,
	height: 60,
	display: 'flex',
	alignItems: 'center',
	justifyContent: 'center',
	border: `3px solid ${theme.palette[versionColor]?.main || theme.palette.primary.main}`,
	backgroundColor: theme.palette.background.paper,
	boxShadow: `0 0 20px ${theme.palette[versionColor]?.main || theme.palette.primary.main}40`,
	'&:hover': {
		transform: 'scale(1.2) rotate(360deg)',
		boxShadow: `0 0 30px ${theme.palette[versionColor]?.main || theme.palette.primary.main}60`,
	},
	'& .MuiSvgIcon-root': {
		fontSize: '1.8rem',
		color: theme.palette[versionColor]?.main || theme.palette.primary.main,
	}
}));

export const VersionCard = styled(Card)(({ theme, versionColor, index }) => ({
	maxWidth: 380,
	width: '100%',
	transition: 'all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275)',
	background: 'linear-gradient(145deg, rgba(255,255,255,0.1), rgba(255,255,255,0.05))',
	backdropFilter: 'blur(10px)',
	border: `1px solid ${theme.palette[versionColor]?.main || theme.palette.primary.main}30`,
	borderRadius: theme.spacing(2),
	position: 'relative',
	overflow: 'hidden',
	marginLeft: index % 2 === 1 ? 'auto' : 0,
	marginRight: index % 2 === 1 ? 0 : 'auto',
	'&::before': {
		content: '""',
		position: 'absolute',
		top: 0,
		left: 0,
		right: 0,
		height: '4px',
		background: `linear-gradient(45deg, ${theme.palette[versionColor]?.main || theme.palette.primary.main}, ${theme.palette[versionColor]?.light || theme.palette.primary.light})`,
	},
	'&:hover': {
		transform: 'translateY(-8px) scale(1.02)',
		boxShadow: `0 20px 40px ${theme.palette[versionColor]?.main || theme.palette.primary.main}30`,
		'& .version-chip': {
			transform: 'scale(1.1)',
		},
		'& .feature-item': {
			transform: 'translateX(5px)',
		}
	}
}));

export const GradientChip = styled(Chip)(({ theme, versionColor }) => ({
	fontWeight: 'bold',
	fontSize: '0.9rem',
	background: `linear-gradient(45deg, ${theme.palette[versionColor]?.main || theme.palette.primary.main}, ${theme.palette[versionColor]?.light || theme.palette.primary.light})`,
	color: 'white',
	boxShadow: '0 4px 15px rgba(0,0,0,0.2)',
	transition: 'all 0.3s ease',
	'&:hover': {
		transform: 'scale(1.05)',
	}
}));

export const HighlightBox = styled(Box)(({ theme }) => ({
	background: 'linear-gradient(145deg, rgba(33, 150, 243, 0.1), rgba(33, 150, 243, 0.05))',
	border: '1px solid rgba(33, 150, 243, 0.2)',
	borderRadius: theme.spacing(1),
	padding: theme.spacing(1.5),
	marginTop: theme.spacing(2),
	'& .MuiTypography-root': {
		fontSize: '0.8rem',
		color: theme.palette.primary.main,
		fontWeight: 500,
	}
}));

export const AnimatedTitle = styled(Typography)(({ theme }) => ({
	textAlign: 'center',
	fontWeight: 'bold',
	background: 'linear-gradient(-45deg, #2196F3, #21CBF3, #9C27B0, #E1BEE7)',
	backgroundSize: '400% 400%',
	WebkitBackgroundClip: 'text',
	WebkitTextFillColor: 'transparent',
	marginBottom: theme.spacing(4),
	animation: 'gradientShift 4s ease infinite, fadeInUp 1s ease-out',
	fontSize: { xs: '2rem', md: '3rem' }
}));

export const OppositeContent = styled(Box)(({ theme, index }) => ({
	margin: 'auto 0',
	textAlign: index % 2 === 0 ? 'right' : 'left',
	animation: 'slideInFromSide 0.8s ease-out',
	paddingLeft: index % 2 === 0 ? theme.spacing(2) : 0,
	paddingRight: index % 2 === 0 ? 0 : theme.spacing(2),
}));
