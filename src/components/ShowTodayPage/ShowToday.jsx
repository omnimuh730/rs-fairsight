import React from 'react';
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { PieChart } from '@mui/x-charts/PieChart';
import { Button, Card, CardContent, Typography, Box } from "@mui/material";
import { ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR } from '../../utils/colorSetting';

const ColorBar = ({ percentages, colors }) => {
    const total = percentages.reduce((sum, percent) => sum + percent, 0);

    return (
        <Box sx={{ display: 'flex', width: '100%', height: '20px', borderRadius: '10px', overflow: 'hidden', boxShadow: '0 2px 5px rgba(0,0,0,0.1)' }}>
            {percentages.map((percent, index) => {
                const width = (percent / total) * 100;
                return (
                    <Box
                        key={index}
                        sx={{
                            width: `${width}%`,
                            backgroundColor: colors[index],
                            transition: 'width 0.3s ease-in-out',
                        }}
                    />
                );
            })}
        </Box>
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
        return () => clearInterval(interval);
    }, []);

    const parseLog = (log) => {
        const lines = log.split("\n").filter((line) => line.trim());
        let notRunDuration = 0;
        let inactiveDuration = 0;
        let activeDuration = 0;

        const colorSlot = [];
        const percentSlot = [];

        for (let i = 0; i < lines.length; i++) {
            const [state, timeRange] = lines[i].split(": ");
            const [start, end] = timeRange.split(" - ").map((t) => {
                const [h, m, s] = t.split(":").map(Number);
                return h * 3600 + m * 60 + s;
            });
            const duration = end - start;

            if (state === "Not run") {
                colorSlot.push(NOTRUN_COLOR);
                percentSlot.push(duration / 864000);
                notRunDuration += duration;
            } else if (state === "Inactive") {
                colorSlot.push(INACTIVE_COLOR);
                percentSlot.push(duration / 864000);
                inactiveDuration += duration;
            } else if (state === "Active") {
                colorSlot.push(ACTIVE_COLOR);
                percentSlot.push(duration / 864000);
                activeDuration += duration;
            }
        }

        setTrackedColorSlot(colorSlot);
        setTrackedDurationSlot(percentSlot);
        setActiveInfo({ Active: activeDuration, Inactive: inactiveDuration, Norun: notRunDuration });
    };

    async function syncTimeData() {
        try {
            const today = new Date();
            const reportDateFormattedText = `rs-fairsight(${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}).txt`;
            const data = await invoke("sync_time_data", { reportDate: reportDateFormattedText });
            parseLog(data);
        } catch (error) {
            console.error("Error fetching sync_time_data:", error);
        }
    }

    return (
        <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center'}}>
    <Card sx={{ maxWidth: 600, boxShadow: '0 4px 20px rgba(0,0,0,0.1)', borderRadius: '15px' }}>
        <CardContent>
            <Typography variant="h5" sx={{ fontWeight: 'bold', mb: 2, textAlign: 'center', color: '#333' }}>
                Today's Activity
            </Typography>
            <Box sx={{ display: 'flex', justifyContent: 'center', mb: 3 }}>
                <Button
                    variant="contained"
                    onClick={syncTimeData}
                    sx={{
                        backgroundColor: '#1976d2',
                        borderRadius: '20px',
                        padding: '8px 20px',
                        textTransform: 'none',
                        '&:hover': { backgroundColor: '#115293' },
                    }}
                >
                    Sync Now
                </Button>
            </Box>
            <Box>
                <ColorBar percentages={trackedDurationSlot} colors={trackedColorSlot} />
            </Box>
            <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: 350 }}> {/* Fixed height to match PieChart */}
                <PieChart
                    colors={[ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR]}
                    series={[
                        {
                            data: [
                                {
                                    id: 0,
                                    value: activeInfo.Active,
                                    label: `Active: ${Math.floor(activeInfo.Active / 3600)}h ${Math.floor((activeInfo.Active % 3600) / 60)}m`,
                                },
                                {
                                    id: 1,
                                    value: activeInfo.Inactive,
                                    label: `Inactive: ${Math.floor(activeInfo.Inactive / 3600)}h ${Math.floor((activeInfo.Inactive % 3600) / 60)}m`,
                                },
                                {
                                    id: 2,
                                    value: activeInfo.Norun,
                                    label: `Not Tracked: ${Math.floor(activeInfo.Norun / 3600)}h ${Math.floor((activeInfo.Norun % 3600) / 60)}m`,
                                },
                            ],
                            innerRadius: 40,
                            outerRadius: 120,
                            paddingAngle: 2,
                            cornerRadius: 8,
                            startAngle: -90,
                            endAngle: 270,
                            cx: '62%', // Center of the pie chart
                            cy: '50%', // Center of the pie chart
                        },
                    ]}
                    width={500}
                    height={350}
                    slotProps={{
                        legend: {
                            direction: 'row',
                            position: { vertical: 'bottom', horizontal: 'center' },
                            padding: 10,
                        },
                    }}
                />
            </Box>
        </CardContent>
    </Card>
    </Box>
    );
};

export default ShowTodayCard;