/**
 * Date utilities for consistent local timezone handling
 * Ensures all date operations use local time consistently
 */

/**
 * Get current date in local timezone formatted as YYYY-MM-DD
 * @returns {string} Date string in YYYY-MM-DD format
 */
export const getLocalDateString = () => {
    const now = new Date();
    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, '0');
    const day = String(now.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
};

/**
 * Convert a Date object to local date string (YYYY-MM-DD)
 * @param {Date} date - The date to convert
 * @returns {string} Date string in YYYY-MM-DD format
 */
export const toLocalDateString = (date) => {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
};

/**
 * Get a date N days ago in local timezone
 * @param {number} daysAgo - Number of days to subtract
 * @returns {Date} Date object representing the date N days ago
 */
export const getDateDaysAgo = (daysAgo) => {
    const date = new Date();
    date.setDate(date.getDate() - daysAgo);
    return date;
};

/**
 * Get dates in range (inclusive) in local timezone
 * @param {Date|string} startDate - Start date
 * @param {Date|string} endDate - End date
 * @returns {string[]} Array of date strings in YYYY-MM-DD format
 */
export const getDatesInRange = (startDate, endDate) => {
    const start = typeof startDate === 'string' ? new Date(startDate) : new Date(startDate);
    const end = typeof endDate === 'string' ? new Date(endDate) : new Date(endDate);
    
    // Normalize to start of day in local timezone
    start.setHours(0, 0, 0, 0);
    end.setHours(0, 0, 0, 0);
    
    const dates = [];
    const currentDate = new Date(start);
    
    while (currentDate <= end) {
        dates.push(toLocalDateString(currentDate));
        currentDate.setDate(currentDate.getDate() + 1);
    }
    
    return dates;
};

/**
 * Default date ranges for UI components using local time
 */
export const getDefaultDateRange = () => [
    {
        startDate: getDateDaysAgo(7),
        endDate: new Date(),
        key: 'selection',
    },
];

export const getStaticRanges = (theme) => [
    {
        label: 'Last 7 Days',
        range: () => ({
            startDate: getDateDaysAgo(6), // 7 days inclusive
            endDate: new Date(),
        }),
        isSelected(range) { return false; }
    },
    {
        label: 'Last 30 Days',
        range: () => ({
            startDate: getDateDaysAgo(29), // 30 days inclusive
            endDate: new Date(),
        }),
        isSelected(range) { return false; }
    },
];
