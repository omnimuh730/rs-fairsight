import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const ColorBar = ({ percentages, colors }) => {
	const total = percentages.reduce((sum, percent) => sum + percent, 0);

	return (
		<div style={{ display: 'flex', width: '100%', height: '20px' }}>
			{percentages.map((percent, index) => {
				const width = (percent / total) * 100;
				return (
					<div
						key={index}
						style={{
							width: `${width}%`,
							backgroundColor: colors[index],
						}}
					></div>
				);
			})}
		</div>
	);
};


function App() {

	const [trackedColorSlot, setTrackedColorSlot] = useState([]);
	const [trackedDurationSlot, setTrackedDurationSlot] = useState([]);

	const [activeInfo, setActiveInfo] = useState({
		Active: 0,
		Inactive: 0,
		Norun: 86400,
	});


	// Function to parse the log and calculate durations
	const parseLog = (log) => {
		const lines = log.split("\n").filter((line) => line.trim()); // Split into lines and remove empty ones
		let notRunDuration = 0;
		let inactiveDuration = 0;
		let activeDuration = 0;

		const colorSlot = [];
		const percentSlot = [];

		for (let i = 0; i < lines.length; i++) {
			const [state, timeRange] = lines[i].split(": ");
			const [start, end] = timeRange.split(" - ").map((t) => {
				const [h, m, s] = t.split(":").map(Number);
				return h * 3600 + m * 60 + s; // Convert to seconds
			});
			const duration = end - start;

			//	  console.log(state, start, end, duration);

			if (state === "Not run") {
				colorSlot.push("#ffff00");
				percentSlot.push(duration / 864000);
				notRunDuration += duration;
			}
			else if (state === "Inactive") {
				colorSlot.push("#ff0000");
				percentSlot.push(duration / 864000);
				inactiveDuration += duration;
			}
			else if (state === "Active") {
				colorSlot.push("#0000ff");
				percentSlot.push(duration / 864000);
				activeDuration += duration;
			}
		}

		//	console.log(colorSlot, percentSlot);
		setTrackedColorSlot(colorSlot);
		setTrackedDurationSlot(percentSlot);

		// Calculate percentages
		const notRunPercent = notRunDuration;
		const inactivePercent = inactiveDuration;
		const activePercent = activeDuration;

		setActiveInfo({ Active: activePercent, Inactive: inactivePercent, Norun: notRunPercent });
	};

	async function syncTimeData() {
		try {
			const data = await invoke("sync_time_data"); // Fetch data from Tauri
			// Assuming data is a string like the log you provided
			parseLog(data);
			//      setTimeData(processedData); // Update state with parsed data
		} catch (error) {
			console.error("Error fetching sync_time_data:", error);
		}
	}

	return (
		<main className="container">
			<button onClick={syncTimeData}>Sync</button>

			<ColorBar percentages={trackedDurationSlot} colors={trackedColorSlot} />
			<p>
								Active: {activeInfo.Active}
				InActive: {activeInfo.Inactive}
				Not run: {activeInfo.Norun}
			</p>
		</main>
	);
}

export default App;