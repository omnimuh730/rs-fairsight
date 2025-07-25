export const getDatesInRange = (startDate, endDate) => {
	let currentDate = new Date(startDate);
	currentDate.setHours(0, 0, 0, 0); // Normalize start date
	const end = new Date(endDate);
	end.setHours(0, 0, 0, 0); // Normalize end date
	let dates = [];
	while (currentDate <= end) {
		const formattedDate = currentDate.toISOString().split('T')[0];
		dates.push(formattedDate);
		currentDate.setDate(currentDate.getDate() + 1);
	}
	return dates;
};

export const parseLog = (log) => {
	// Handle potential null/undefined log input gracefully
	if (!log) {
		console.warn("Received null or undefined log data.");
		// Return a default structure, maybe all notrun?
		return JSON.stringify({ active: 0, inactive: 0, notrun: 86400 });
	}
	if (log.includes("found")) { // Assuming "not found" means no log file?
		return JSON.stringify({ active: 0, inactive: 0, notrun: 86400 });
	}
	const lines = log.split("\n").filter((line) => line.trim());
	if (lines.length === 0) {
		// Handle empty log file after filtering lines
		return JSON.stringify({ active: 0, inactive: 0, notrun: 86400 });
	}
	let activeDuration = 0;
	let inactiveDuration = 0;
	let notRunDuration = 0; // This likely needs calculation based on total time

	for (let i = 0; i < lines.length; i++) {
		// Add robust parsing with error handling
		try {
			const parts = lines[i].split(": ");
			if (parts.length !== 2) continue; // Skip malformed lines
			const state = parts[0];
			const timeRange = parts[1];
			if (!timeRange || !timeRange.includes(" - ")) continue; // Check time range format

			const [startStr, endStr] = timeRange.split(" - ");
			const startParts = startStr?.split(":").map(Number);
			const endParts = endStr?.split(":").map(Number);

			if (!startParts || startParts.length !== 3 || startParts.some(isNaN) ||
				!endParts || endParts.length !== 3 || endParts.some(isNaN)) {
				console.warn("Skipping malformed time string:", lines[i]);
				continue; // Skip if time parsing fails
			}

			const start = startParts[0] * 3600 + startParts[1] * 60 + startParts[2];
			const end = endParts[0] * 3600 + endParts[1] * 60 + endParts[2];

			// Handle cases where end time might be on the next day (e.g. 23:59:59 - 00:00:05)
			// This simple parser assumes all times are within the same 24h period.
			// If logs can span midnight, parsing needs to be more complex.
			if (end < start) {
				console.warn(`End time (${endStr}) is before start time (${startStr}). Assuming single day log. Skipping: ${lines[i]}`);
				continue; // Or handle wrap-around if necessary
			}

			const duration = end - start;

			if (state === "Active") activeDuration += duration;
			else if (state === "Inactive") inactiveDuration += duration;
			// Remove "Not run" parsing from here - it should be calculated
		} catch (e) {
			console.error("Error parsing log line:", lines[i], e);
		}
	}

	// Calculate notRunDuration: total seconds in a day minus active and inactive
	const totalSecondsInDay = 24 * 60 * 60;
	// Ensure calculated notRun isn't negative due to small parsing errors or overlap
	notRunDuration = Math.max(0, totalSecondsInDay - activeDuration - inactiveDuration);

	// Optional: Sanity check if total duration exceeds a day
	if (activeDuration + inactiveDuration + notRunDuration > totalSecondsInDay + 60) { // Allow 1 min buffer for rounding
		console.warn(`Total calculated duration (${activeDuration + inactiveDuration + notRunDuration}s) exceeds 24 hours for log.`);
		// Adjust normalization strategy if needed, e.g., cap at totalSecondsInDay
	}

	return JSON.stringify({
		active: activeDuration,
		inactive: inactiveDuration,
		notrun: notRunDuration,
	});
};
