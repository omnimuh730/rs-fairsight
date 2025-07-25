import { createTheme } from '@mui/material/styles';

export const theme = createTheme({
	palette: {
		mode: 'light',
		primary: { main: '#00e676' },
		background: { default: '#FFFFFF', paper: '#EAEAEA' },
	},
	typography: {
		fontFamily: 'Inter, sans-serif',
		h5: { fontWeight: 600 },
	},
	components: {
		MuiPaper: {
			styleOverrides: {
				root: {
					borderRadius: 12,
					boxShadow: '0 4px 20px rgba(36, 156, 255, 0.58)',
				},
			},
		},
	},
});
