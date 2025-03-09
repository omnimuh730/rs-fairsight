import React from 'react';
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { PieChart } from '@mui/x-charts/PieChart';

import { Button } from "@mui/material";

import { ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR } from '../../utils/colorSetting';

const ColorBar = ({ percentages, colors }) => {
    const total = percentages.reduce((sum, percent) => sum + percent, 0);

    return (
        <div style={{ display: 'flex', width: '100%', height: '40px' }}>
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
const ShowTodayCard = () => {

    const [trackedColorSlot, setTrackedColorSlot] = useState([]);
    const [trackedDurationSlot, setTrackedDurationSlot] = useState([]);

    const [activeInfo, setActiveInfo] = useState({
        Active: 0,
        Inactive: 0,
        Norun: 86400,
    });
    useEffect(() => {
        const interval = setInterval(syncTimeData, 10000);
        return () => clearInterval(interval); // Cleanup on unmount
    }, []);


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
                colorSlot.push(NOTRUN_COLOR);
                percentSlot.push(duration / 864000);
                notRunDuration += duration;
            }
            else if (state === "Inactive") {
                colorSlot.push(INACTIVE_COLOR);
                percentSlot.push(duration / 864000);
                inactiveDuration += duration;
            }
            else if (state === "Active") {
                colorSlot.push(ACTIVE_COLOR);
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
            // Get today's date
            const today = new Date();

            // Format the date as YYYY-MM-DD.txt
            const reportDateFormattedText = `rs-fairsight(${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}).txt`;
            const data = await invoke("sync_time_data", { reportDate: reportDateFormattedText}); // Fetch data from Tauri
            // Assuming data is a string like the log you provided
            parseLog(data);
            //      setTimeData(processedData); // Update state with parsed data
        } catch (error) {
            console.error("Error fetching sync_time_data:", error);
        }
    }
    return (

        <main className="container">
            <Button variant="contained" onClick={syncTimeData}>Sync</Button>
            <ColorBar percentages={trackedDurationSlot} colors={trackedColorSlot} />

            <PieChart
                colors={[
                    ACTIVE_COLOR, // Active
                    INACTIVE_COLOR, // Inactive
                    NOTRUN_COLOR, // Not Running
                ]}
                series={[
                    {
                        data: [
                            {
                                id: 0,
                                value: activeInfo.Active,
                                label: `Active: ${Math.floor(activeInfo.Active / 3600)}h ${Math.floor((activeInfo.Active % 3600) / 60)}m ${activeInfo.Active % 60}s`
                            },
                            {
                                id: 1,
                                value: activeInfo.Inactive,
                                label: `InActive: ${Math.floor(activeInfo.Inactive / 3600)}h ${Math.floor((activeInfo.Inactive % 3600) / 60)}m ${activeInfo.Inactive % 60}s`
                            },
                            {
                                id: 2,
                                value: activeInfo.Norun,
                                label: `Not Tracked: ${Math.floor(activeInfo.Norun / 3600)}h ${Math.floor((activeInfo.Norun % 3600) / 60)}m ${activeInfo.Norun % 60}s`
                            },
                        ],
                        innerRadius: 30,
                        outerRadius: 100,
                        paddingAngle: 1,
                        cornerRadius: 5,
                        startAngle: -45,
                        endAngle: 360,
                        cx: 150,
                        cy: 150,
                    }
                ]}
                width={550}
                height={300}
            />
        </main>
    )
}

export default ShowTodayCard;