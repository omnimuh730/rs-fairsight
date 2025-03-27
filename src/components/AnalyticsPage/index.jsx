import { listen } from '@tauri-apps/api/event'; // Note: Ensure this is "@tauri-apps/api/event"
import { useEffect } from 'react';

const AnalyticsPage = () => {
	useEffect(() => {
		let unlistenFn; // Variable to store the unlisten function

		// Immediately invoked async function to handle the Promise
		(async () => {
			unlistenFn = await listen('my-event', (event) => {
				console.log(event.payload);
			});
		})();

		// Cleanup function
		return () => {
			if (unlistenFn) {
				unlistenFn(); // Call the resolved unlisten function
			}
		};
	}, []);

	return (
		<div>
			<h1>Analytics</h1>
			<p>Welcome to the analytics page!</p>
			<p>Here you can view your application's performance metrics and trends.</p>
		</div>
	);
};

export default AnalyticsPage;