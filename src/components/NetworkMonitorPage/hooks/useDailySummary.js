import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { format } from 'date-fns';

export const useDailySummary = (date) => {
  const [summary, setSummary] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchSummary = async () => {
      try {
        setLoading(true);
        setError(null);
        const dateString = format(date, 'yyyy-MM-dd');
        const result = await invoke('load_daily_summary_command', { date: dateString });
        setSummary(result);
      } catch (err) {
        setError(err.toString());
        console.error(`Failed to fetch daily summary for ${date}:`, err);
      } finally {
        setLoading(false);
      }
    };

    fetchSummary();
  }, [date]);

  return { summary, loading, error };
};