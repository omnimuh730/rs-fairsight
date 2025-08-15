import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { format } from 'date-fns';

export const useDailySummary = (dateString) => {
  const [summary, setSummary] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    if (!dateString) return;

    const fetchSummary = async () => {
      try {
        setLoading(true);
        setError(null);
        const result = await invoke('load_daily_summary_command', { date: dateString });
        setSummary(result);
      } catch (err) {
        setError(err.toString());
        console.error(`Failed to fetch daily summary for ${dateString}:`, err);
        setSummary(null); // Clear summary on error
      } finally {
        setLoading(false);
      }
    };

    fetchSummary();
  }, [dateString]);

  const refetch = async () => {
      if (!dateString) return;
      setLoading(true);
      try {
        const result = await invoke('load_daily_summary_command', { date: dateString });
        setSummary(result);
      } catch (err) {
        setError(err.toString());
      } finally {
        setLoading(false);
      }
  };

  return { summary, loading, error, refetch };
};