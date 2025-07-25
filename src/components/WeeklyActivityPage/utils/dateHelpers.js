export const getDefaultDateRange = () => [
	{
		// Initialize with a default range, e.g., last 7 days
		startDate: new Date(new Date().setDate(new Date().getDate() - 7)),
		endDate: new Date(),
		key: 'selection',
	},
];

export const getStaticRanges = (theme) => [
	{
		label: 'Last 7 Days',
		range: () => ({
			startDate: new Date(new Date().setDate(new Date().getDate() - 6)), // Correct calculation for 7 days inclusive
			endDate: new Date(),
		}),
		isSelected(range) { return false; }
	},
	{
		label: 'Last 30 Days',
		range: () => ({
			startDate: new Date(new Date().setDate(new Date().getDate() - 29)), // Correct calculation for 30 days inclusive
			endDate: new Date(),
		}),
		isSelected(range) { return false; }
	},
];
