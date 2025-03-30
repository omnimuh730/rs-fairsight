import React, { useState, useEffect } from 'react'; // Added useEffect
import { DateRangePicker } from 'react-date-range';
import { invoke } from "@tauri-apps/api/core";
import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';
import { Box, Typography, Paper, CssBaseline } from '@mui/material';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { LineChart } from '@mui/x-charts/LineChart';
import { ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR } from '../../utils/colorSetting';

// Define a modern theme (keep your theme definition)
const theme = createTheme({
    palette: {
        mode: 'light', // Consider 'dark' if you prefer dark mode
        primary: { main: '#00e676' }, // Vibrant green accent
        background: { default: '#FFFFFF', paper: '#EAEAEA' },
    },
    typography: {
        fontFamily: 'Inter, sans-serif', // Modern font
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


const DateRangePickerComponent = () => {
    const [dateRange, setDateRange] = useState([
        {
            // Initialize with a default range, e.g., last 7 days
            startDate: new Date(new Date().setDate(new Date().getDate() - 7)),
            endDate: new Date(),
            key: 'selection',
        },
    ]);
    const [aggregateTimeList, setAggregateTimeList] = useState([]);
    const [isLoading, setIsLoading] = useState(false); // Added loading state

    // Function to format dates (moved outside handleDateChange for reusability)
    const getDatesInRange = (startDate, endDate) => {
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

    const parseLog = (log) => {
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

        let totalParsedDuration = 0;

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
                totalParsedDuration += duration;

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

    async function aggregateDaysReport(timeList) {
        setIsLoading(true); // Set loading true
        setAggregateTimeList([]); // Clear previous data immediately
        try {
            if (!Array.isArray(timeList) || timeList.length === 0) {
                 // Don't throw error, just set empty data if range is invalid or empty
                 console.log("Time list is empty or invalid.");
                 setAggregateTimeList([]);
                 setIsLoading(false);
                 return;
            }
            // Ensure timeList contains valid date strings if needed by backend
            console.log("Fetching logs for dates:", timeList);
            const data = await invoke("aggregate_week_activity_logs", { dataList: timeList });
             console.log("Raw data received:", data); // Log raw backend response

            if (!Array.isArray(data)) {
                throw new Error("Received invalid data structure from backend.");
            }

            let activityReport = [];
            for (let i = 0; i < data.length; i++) {
                const dailyActivity = parseLog(data[i]); // Parse each day's log string
                activityReport.push(dailyActivity);
            }
             console.log("Parsed activity report:", activityReport); // Log parsed data
            setAggregateTimeList(activityReport);
        } catch (error) {
            console.error("Error fetching or processing aggregate_week_activity_logs:", error);
            setAggregateTimeList([]); // Clear data on error
        } finally {
             setIsLoading(false); // Set loading false
        }
    }

     // Fetch data when the component mounts or dateRange changes
     useEffect(() => {
        if (dateRange[0].startDate && dateRange[0].endDate) {
            const dates = getDatesInRange(dateRange[0].startDate, dateRange[0].endDate);
            aggregateDaysReport(dates);
        }
     }, [dateRange]); // Depend on dateRange state

    const handleDateChange = (ranges) => {
        const { startDate, endDate } = ranges.selection;
         // Update the state to trigger the useEffect hook
        setDateRange([{
            startDate: startDate || new Date(), // Fallback if null
            endDate: endDate || new Date(), // Fallback if null
            key: 'selection',
        }]);
         // Data fetching is now handled by useEffect
    };


    // Prepare chart data, ensuring robustness against parsing errors
    const chartSeriesData = aggregateTimeList.map(item => {
        try {
            return JSON.parse(item);
        } catch (e) {
            console.error("Failed to parse item for chart:", item, e);
            // Return default zero values if parsing fails for an item
            return { active: 0, inactive: 0, notrun: 0 };
        }
    });

    // Generate x-axis labels based on the actual dates used for fetching
    const xAxisLabels = getDatesInRange(dateRange[0].startDate, dateRange[0].endDate)
        .map(dateStr => {
            const date = new Date(dateStr + 'T00:00:00'); // Ensure correct date parsing
            return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
        });

    // Ensure chart data length matches x-axis labels length
     // This might happen if backend returns fewer/more items than requested dates
     const dataLength = Math.min(chartSeriesData.length, xAxisLabels.length);


    const chartData = {
        series: [
            {
                id: 'active',
                // Use dataLength to slice data arrays
                data: chartSeriesData.slice(0, dataLength).map(item => (item.active || 0) / 3600),
                label: 'Active',
                area: true,
                stack: 'total',
                color: ACTIVE_COLOR || '#4caf50', // Provide fallback color
                curve: 'monotoneX', // Changed curve type
                showMark: false,
            },
            {
                id: 'inactive',
                data: chartSeriesData.slice(0, dataLength).map(item => (item.inactive || 0) / 3600),
                label: 'Inactive',
                area: true,
                stack: 'total',
                color: INACTIVE_COLOR || '#ff9800', // Provide fallback color
                curve: 'monotoneX', // Changed curve type
                showMark: false,
            },
            {
                id: 'notrun',
                data: chartSeriesData.slice(0, dataLength).map(item => (item.notrun || 0) / 3600),
                label: 'Not Run',
                area: true,
                stack: 'total',
                color: NOTRUN_COLOR || '#9e9e9e', // Provide fallback color
                curve: 'monotoneX', // Changed curve type
                showMark: false,
            },
        ],
        xAxis: [{
            // Use the generated labels, sliced to match dataLength
            data: xAxisLabels.slice(0, dataLength),
            scaleType: 'point',
            id: 'dates',
        }],
        yAxis: [{ label: 'Hours' }],
        height: 300,
        sx: { // Correct sx for MUI X Charts styling if needed
             '.MuiLineElement-root': { strokeWidth: 2 }, // Example: Thinner line
             '.MuiAreaElement-root': { opacity: 0.6 },  // Example: Slightly more opacity
        },
        slotProps: {
            legend: { // Keep legend hidden if desired
                hidden: true,
            },
        },
    };

    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Box sx={{ p: { xs: 1, sm: 2, md: 4 }, bgcolor: 'background.default', minHeight: '100vh' }}>
                <Typography variant="h5" color="textPrimary" gutterBottom align="center" sx={{ mb: 3 }}>
                     Activity Dashboard
                </Typography>

                 {/* Center the Date Picker */}
                 <Box display="flex" justifyContent="center" mb={4}>
                     <Paper elevation={3} sx={{ p: { xs: 1, sm: 2 }, width: 'fit-content' /* Adjust width */ }}>
                        <DateRangePicker
                            ranges={dateRange}
                            onChange={handleDateChange}
                            showDateDisplay={false} // Keep this false for cleaner look
                            direction="vertical" // Consider vertical on smaller screens if needed
                            months={1}
                            rangeColors={[theme.palette.primary.main]} // Use theme color
                            inputRanges={[]} // Hide custom input ranges
                            staticRanges={[ // Simplified static ranges
                                {
                                     label: 'Last 7 Days',
                                     range: () => ({
                                        startDate: new Date(new Date().setDate(new Date().getDate() - 6)), // Correct calculation for 7 days inclusive
                                        endDate: new Date(),
                                    }),
                                     isSelected(range) { /* Optional: logic to highlight if selected */ return false; }
                                },
                                {
                                    label: 'Last 30 Days',
                                    range: () => ({
                                        startDate: new Date(new Date().setDate(new Date().getDate() - 29)), // Correct calculation for 30 days inclusive
                                        endDate: new Date(),
                                    }),
                                     isSelected(range) { return false; }
                                },
                                // Add more ranges like 'This Month', 'Last Month' if needed
                            ]}
                         />
                    </Paper>
                </Box>

                 {/* Conditional Rendering for Chart based on loading and data */}
                 <Box display="flex" justifyContent="center">
                     {isLoading ? (
                         <Typography>Loading chart data...</Typography>
                     ) : aggregateTimeList.length > 0 && xAxisLabels.length > 0 && dataLength > 0 ? (
                          <Paper elevation={3} sx={{ p: { xs: 1, sm: 2, md: 3 }, width: '100%', maxWidth: 1000 /* Ensure max width */}}>
                            <LineChart
                                 {...chartData} // Spread calculated chartData
                                margin={{ top: 20, right: 30, bottom: 50, left: 60 }} // Adjusted margins
                                // --- THIS IS THE KEY CHANGE ---
                                tooltip={{ trigger: 'axis' }} // Change trigger to 'axis'
                                // Removed the incorrect sx block targeting recharts
                            />
                        </Paper>
                    ) : !isLoading ? ( // Only show 'No data' if not loading
                         <Typography>No activity data available for the selected range.</Typography>
                     ) : null /* Avoid rendering anything else during load */}
                </Box>
            </Box>
        </ThemeProvider>
    );
};

export default DateRangePickerComponent;

// --- Dummy color settings if not imported ---
// const ACTIVE_COLOR = '#4caf50'; // Green
// const INACTIVE_COLOR = '#ff9800'; // Orange
// const NOTRUN_COLOR = '#9e9e9e'; // Grey