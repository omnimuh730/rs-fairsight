import Layout from './components/layout/index';
import Box from '@mui/material/Box';

function App() {

	return (
		<div>
			<Box component="section" sx={{ p: 2, border: '1px dashed grey' }}>
				<Layout/>
			</Box>
		</div>
	);
}

export default App;