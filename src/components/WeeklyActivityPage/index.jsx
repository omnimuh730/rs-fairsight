import React, { useState } from 'react';
import { DateRangePicker } from 'react-date-range';
import { invoke } from "@tauri-apps/api/core";
import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';
import { Box, Typography, Paper, CssBaseline } from '@mui/material';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { LineChart } from '@mui/x-charts/LineChart';
import { ACTIVE_COLOR, INACTIVE_COLOR, NOTRUN_COLOR } from '../../utils/colorSetting';

// Define a modern theme
const theme = createTheme({
  palette: {
    mode: 'light', // Dark mode for a sleek look
    primary: { main: '#00e676' }, // Vibrant green accent
    background: { default: '#FFFFFFFF', paper: '#EAEAEAFF' },
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
          boxShadow: '0 4px 20px rgba(0, 0, 0, 0.3)',
        },
      },
    },
  },
});

const DateRangePickerComponent = () => {
  const [dateRange, setDateRange] = useState([
    {
      startDate: new Date(),
      endDate: new Date(),
      key: 'selection',
    },
  ]);
  const [aggregateTimeList, setAggregateTimeList] = useState([]);
  const [selectedItem, setSelectedItem] = useState(null);

  const handleDateChange = async (ranges) => {
    const { startDate, endDate } = ranges.selection;
    setDateRange([ranges.selection]);

    let currentDate = new Date(startDate);
    let dates = [];
    while (currentDate <= endDate) {
      const formattedDate = currentDate.toISOString().split('T')[0];
      dates.push(formattedDate);
      currentDate.setDate(currentDate.getDate() + 1);
    }
    await aggregateDaysReport(dates);
  };

  const parseLog = (log) => {
    if (log.includes("found")) {
      return JSON.stringify({ active: 0, inactive: 0, notrun: 86400 });
    }
    const lines = log.split("\n").filter((line) => line.trim());
    let activeDuration = 0;
    let inactiveDuration = 0;
    let notRunDuration = 0;

    for (let i = 0; i < lines.length; i++) {
      const [state, timeRange] = lines[i].split(": ");
      const [start, end] = timeRange.split(" - ").map((t) => {
        const [h, m, s] = t.split(":").map(Number);
        return h * 3600 + m * 60 + s;
      });
      const duration = end - start;
      if (state === "Active") activeDuration += duration;
      else if (state === "Inactive") inactiveDuration += duration;
      else if (state === "Not run") notRunDuration += duration;
    }
    return JSON.stringify({
      active: activeDuration,
      inactive: inactiveDuration,
      notrun: notRunDuration,
    });
  };

  async function aggregateDaysReport(timeList) {
    try {
      if (!Array.isArray(timeList)) throw new Error("timeList must be an array");
      const data = await invoke("aggregate_week_activity_logs", { dataList: timeList });
      let activityReport = [];
      for (let i = 0; i < data.length; i++) {
        const dailyActivity = parseLog(data[i]);
        activityReport.push(dailyActivity);
      }
      setAggregateTimeList(activityReport);
      return data;
    } catch (error) {
      console.error("Error fetching sync_time_data:", error);
    }
  }

  const chartData = {
    series: [
      {
        id: 'active',
        data: aggregateTimeList.map(item => JSON.parse(item).active / 3600), // Convert to hours
        label: 'Active',
        area: true,
        stack: 'total',
        color: ACTIVE_COLOR,
        curve: 'linear',
        showMark: false,
      },
      {
        id: 'inactive',
        data: aggregateTimeList.map(item => JSON.parse(item).inactive / 3600), // Convert to hours
        label: 'Inactive',
        area: true,
        stack: 'total',
        color: INACTIVE_COLOR,
        curve: 'linear',
        showMark: false,
      },
      {
        id: 'notrun',
        data: aggregateTimeList.map(item => JSON.parse(item).notrun / 3600), // Convert to hours
        label: 'Not Run',
        area: true,
        stack: 'total',
        color: NOTRUN_COLOR,
        curve: 'linear',
        showMark: false,
      },
    ],
    xAxis: [{
      data: dateRange[0].startDate
        ? Array.from({ length: aggregateTimeList.length }, (_, i) => {
            const date = new Date(dateRange[0].startDate);
            date.setDate(date.getDate() + i);
            return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
          })
        : [],
      scaleType: 'point',
      id: 'dates',
    }],
    yAxis: [{ label: 'Hours' }],
    height: 500,
    sx: {
      '.MuiLineElement-root': { strokeWidth: 2 },
      '.MuiAreaElement-root': { opacity: 0.8 },
    },
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ p: 4, bgcolor: 'background.default', minHeight: '100vh' }}>
        <Typography variant="h5" color="primary" gutterBottom align="center">
          Activity Dashboard
        </Typography>

        <Paper elevation={3} sx={{ p: 3, mb: 4, maxWidth: 600, mx: 'auto' }}>
          <DateRangePicker
            ranges={dateRange}
            onChange={handleDateChange}
            showDateDisplay={false}
            direction="horizontal"
            months={1}
            rangeColors={['#006AB1FF']}
            inputRanges={[]}
            staticRanges={[
              {
                label: 'Last 7 Days',
                range: () => ({
                  startDate: new Date(new Date().setDate(new Date().getDate() - 7)),
                  endDate: new Date(),
                }),
                isSelected: () => false,
              },
              {
                label: 'Last 30 Days',
                range: () => ({
                  startDate: new Date(new Date().setDate(new Date().getDate() - 30)),
                  endDate: new Date(),
                }),
                isSelected: () => false,
              },
            ]}
          />
        </Paper>

        {aggregateTimeList.length > 0 && (
          <Paper elevation={3} sx={{ p: 3, maxWidth: 1000, mx: 'auto' }}>
            <LineChart
              {...chartData}
              margin={{ top: 20, right: 40, bottom: 40, left: 60 }}
              onItemClick={(event, d) => setSelectedItem(d)}
              tooltip={{ trigger: 'item' }}
            />
            {selectedItem && (
              <Typography variant="body2" color="text.secondary" align="center" sx={{ mt: 2 }}>
                {`Selected: ${chartData.series[selectedItem.seriesIndex].label} - ${
                  chartData.xAxis[0].data[selectedItem.dataIndex]
                }: ${chartData.series[selectedItem.seriesIndex].data[selectedItem.dataIndex].toFixed(2)} hours`}
              </Typography>
            )}
          </Paper>
        )}
      </Box>
    </ThemeProvider>
  );
};

export default DateRangePickerComponent;