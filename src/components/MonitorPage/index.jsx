import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  Box,
  Paper,
  Typography,
  Card,
  CardContent,
  Button,
  Chip,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Alert,
  AlertTitle,
  Grid,
  Switch,
  FormControlLabel,
  TextField,
  MenuItem,
  Divider,
  IconButton,
  Tooltip
} from '@mui/material';
import {
  Refresh,
  Clear,
  Download,
  FilterList,
  Info,
  Warning,
  Error,
  BugReport,
  TouchApp
} from '@mui/icons-material';

const MonitorPage = () => {
  const [logs, setLogs] = useState([]);
  const [healthStatus, setHealthStatus] = useState('');
  const [loading, setLoading] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(true);
  const [filter, setFilter] = useState('all');
  const [maxLogs, setMaxLogs] = useState(100);
  const [stats, setStats] = useState({
    total: 0,
    info: 0,
    warning: 0,
    error: 0,
    activity: 0,
    debug: 0
  });

  const fetchLogs = async () => {
    setLoading(true);
    try {
      const recentLogs = await invoke('get_recent_logs_limited', { count: maxLogs });
      setLogs(recentLogs);
      
      // Calculate stats
      const newStats = recentLogs.reduce((acc, log) => {
        acc.total++;
        acc[log.level.toLowerCase()]++;
        return acc;
      }, { total: 0, info: 0, warning: 0, error: 0, activity: 0, debug: 0 });
      
      setStats(newStats);
    } catch (error) {
      console.error('Failed to fetch logs:', error);
    }
    setLoading(false);
  };

  const fetchHealthStatus = async () => {
    try {
      const status = await invoke('get_health_status');
      setHealthStatus(status);
    } catch (error) {
      console.error('Failed to fetch health status:', error);
      setHealthStatus('Error fetching health status');
    }
  };

  const clearLogs = async () => {
    try {
      await invoke('clear_all_logs');
      setLogs([]);
      setStats({ total: 0, info: 0, warning: 0, error: 0, activity: 0, debug: 0 });
    } catch (error) {
      console.error('Failed to clear logs:', error);
    }
  };

  const exportLogs = () => {
    const filteredLogs = getFilteredLogs();
    const csvContent = "data:text/csv;charset=utf-8," 
      + "Timestamp,Level,Module,Message\\n"
      + filteredLogs.map(log => 
          `"${log.timestamp}","${log.level}","${log.module}","${log.message}"`
        ).join("\\n");
    
    const encodedUri = encodeURI(csvContent);
    const link = document.createElement("a");
    link.setAttribute("href", encodedUri);
    link.setAttribute("download", `fairsight-logs-${new Date().toISOString().split('T')[0]}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const getFilteredLogs = () => {
    if (filter === 'all') return logs;
    return logs.filter(log => log.level.toLowerCase() === filter.toLowerCase());
  };

  const getLevelIcon = (level) => {
    switch (level.toLowerCase()) {
      case 'info': return <Info color="info" />;
      case 'warning': return <Warning color="warning" />;
      case 'error': return <Error color="error" />;
      case 'debug': return <BugReport color="action" />;
      case 'activity': return <TouchApp color="success" />;
      default: return <Info />;
    }
  };

  const getLevelColor = (level) => {
    switch (level.toLowerCase()) {
      case 'info': return 'info';
      case 'warning': return 'warning';
      case 'error': return 'error';
      case 'debug': return 'default';
      case 'activity': return 'success';
      default: return 'default';
    }
  };

  const getHealthStatusSeverity = (status) => {
    if (status.includes('Warning') || status.includes('No activity')) return 'warning';
    if (status.includes('Error')) return 'error';
    if (status.includes('working normally')) return 'success';
    return 'info';
  };

  useEffect(() => {
    fetchLogs();
    fetchHealthStatus();
  }, [maxLogs]);

  useEffect(() => {
    if (autoRefresh) {
      const interval = setInterval(() => {
        fetchLogs();
        fetchHealthStatus();
      }, 5000); // Refresh every 5 seconds

      return () => clearInterval(interval);
    }
  }, [autoRefresh, maxLogs]);

  const filteredLogs = getFilteredLogs();

  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h4" gutterBottom>
        System Monitor & Logs
      </Typography>
      
      {/* Health Status Card */}
      <Card sx={{ mb: 3 }}>
        <CardContent>
          <Typography variant="h6" gutterBottom>
            System Health Status
          </Typography>
          <Alert severity={getHealthStatusSeverity(healthStatus)} sx={{ mb: 2 }}>
            <AlertTitle>Time Tracking Status</AlertTitle>
            {healthStatus}
          </Alert>
          
          {/* Statistics Grid */}
          <Grid container spacing={2} sx={{ mt: 2 }}>
            <Grid item xs={6} sm={2}>
              <Paper sx={{ p: 2, textAlign: 'center' }}>
                <Typography variant="h6">{stats.total}</Typography>
                <Typography variant="caption">Total Logs</Typography>
              </Paper>
            </Grid>
            <Grid item xs={6} sm={2}>
              <Paper sx={{ p: 2, textAlign: 'center', bgcolor: 'info.light' }}>
                <Typography variant="h6">{stats.info}</Typography>
                <Typography variant="caption">Info</Typography>
              </Paper>
            </Grid>
            <Grid item xs={6} sm={2}>
              <Paper sx={{ p: 2, textAlign: 'center', bgcolor: 'warning.light' }}>
                <Typography variant="h6">{stats.warning}</Typography>
                <Typography variant="caption">Warnings</Typography>
              </Paper>
            </Grid>
            <Grid item xs={6} sm={2}>
              <Paper sx={{ p: 2, textAlign: 'center', bgcolor: 'error.light' }}>
                <Typography variant="h6">{stats.error}</Typography>
                <Typography variant="caption">Errors</Typography>
              </Paper>
            </Grid>
            <Grid item xs={6} sm={2}>
              <Paper sx={{ p: 2, textAlign: 'center', bgcolor: 'success.light' }}>
                <Typography variant="h6">{stats.activity}</Typography>
                <Typography variant="caption">Activity</Typography>
              </Paper>
            </Grid>
            <Grid item xs={6} sm={2}>
              <Paper sx={{ p: 2, textAlign: 'center', bgcolor: 'grey.300' }}>
                <Typography variant="h6">{stats.debug}</Typography>
                <Typography variant="caption">Debug</Typography>
              </Paper>
            </Grid>
          </Grid>
        </CardContent>
      </Card>

      {/* Controls */}
      <Paper sx={{ p: 2, mb: 3 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, flexWrap: 'wrap' }}>
          <FormControlLabel
            control={
              <Switch
                checked={autoRefresh}
                onChange={(e) => setAutoRefresh(e.target.checked)}
              />
            }
            label="Auto Refresh"
          />
          
          <TextField
            select
            label="Filter Level"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            size="small"
            sx={{ minWidth: 120 }}
          >
            <MenuItem value="all">All Levels</MenuItem>
            <MenuItem value="info">Info</MenuItem>
            <MenuItem value="warning">Warning</MenuItem>
            <MenuItem value="error">Error</MenuItem>
            <MenuItem value="activity">Activity</MenuItem>
            <MenuItem value="debug">Debug</MenuItem>
          </TextField>

          <TextField
            type="number"
            label="Max Logs"
            value={maxLogs}
            onChange={(e) => setMaxLogs(parseInt(e.target.value))}
            size="small"
            inputProps={{ min: 10, max: 1000 }}
            sx={{ width: 120 }}
          />

          <Box sx={{ flexGrow: 1 }} />

          <Tooltip title="Refresh logs">
            <IconButton onClick={fetchLogs} disabled={loading}>
              <Refresh />
            </IconButton>
          </Tooltip>

          <Tooltip title="Export logs to CSV">
            <IconButton onClick={exportLogs}>
              <Download />
            </IconButton>
          </Tooltip>

          <Tooltip title="Clear all logs">
            <IconButton onClick={clearLogs} color="error">
              <Clear />
            </IconButton>
          </Tooltip>
        </Box>
      </Paper>

      {/* Logs Table */}
      <TableContainer component={Paper}>
        <Table stickyHeader>
          <TableHead>
            <TableRow>
              <TableCell>Timestamp</TableCell>
              <TableCell>Level</TableCell>
              <TableCell>Module</TableCell>
              <TableCell>Message</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {filteredLogs.map((log, index) => (
              <TableRow 
                key={index}
                sx={{ 
                  '&:nth-of-type(odd)': { backgroundColor: 'rgba(0, 0, 0, 0.04)' },
                  '&:hover': { backgroundColor: 'rgba(0, 0, 0, 0.08)' }
                }}
              >
                <TableCell sx={{ fontFamily: 'monospace', fontSize: '0.875rem' }}>
                  {log.timestamp}
                </TableCell>
                <TableCell>
                  <Chip
                    icon={getLevelIcon(log.level)}
                    label={log.level}
                    color={getLevelColor(log.level)}
                    size="small"
                    variant="outlined"
                  />
                </TableCell>
                <TableCell sx={{ fontWeight: 'medium' }}>
                  {log.module}
                </TableCell>
                <TableCell sx={{ fontFamily: 'monospace', fontSize: '0.875rem' }}>
                  {log.message}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        
        {filteredLogs.length === 0 && (
          <Box sx={{ p: 4, textAlign: 'center' }}>
            <Typography variant="body1" color="text.secondary">
              {filter === 'all' ? 'No logs available' : `No ${filter} logs found`}
            </Typography>
          </Box>
        )}
      </TableContainer>
    </Box>
  );
};

export default MonitorPage;
