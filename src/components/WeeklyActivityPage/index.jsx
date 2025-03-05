import React, { useState } from 'react';
import { DateRangePicker } from 'react-date-range';
import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';

const DateRangePickerComponent = () => {
    const [dateRange, setDateRange] = useState([
        {
            startDate: new Date(),
            endDate: new Date(),
            key: 'selection',
        }
    ]);

    const [aggregateTimeList, setAggregateTimeList] = useState([]);

    const handleDateChange = (ranges) => {
        const { startDate, endDate } = ranges.selection;
        setDateRange([ranges.selection]);

        // Generate list of dates
        let currentDate = new Date(startDate);
        let dates = [];

        while (currentDate <= endDate) {
            // Format date as YYYY-MM-DD
            const formattedDate = currentDate.toISOString().split('T')[0];
            dates.push(formattedDate);
            currentDate.setDate(currentDate.getDate() + 1);
        }

        setAggregateTimeList(dates);
    };

    return (
        <div>
            <DateRangePicker ranges={dateRange} onChange={handleDateChange} />
            <div>Selected Start Date: {dateRange[0].startDate.toDateString()}</div>
            <div>Selected End Date: {dateRange[0].endDate.toDateString()}</div>
            <div>
                Aggregate Dates:
                <ul>
                    {aggregateTimeList.map((date, index) => (
                        <li key={index}>{date}</li>
                    ))}
                </ul>
            </div>
        </div>
    );
};

export default DateRangePickerComponent;
