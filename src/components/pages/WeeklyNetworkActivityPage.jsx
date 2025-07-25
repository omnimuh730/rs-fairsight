import React, { useState, useEffect } from 'react';
import {
	Card,
	CardContent,
	Typography,
	Box,
	Button,
	Container,
	Paper,
	Grid,
	Alert,
	CircularProgress,
	Divider,
	List,
	ListItem,
	ListItemText,
	Chip
} from '@mui/material';
import {
	DatePicker,
	LocalizationProvider
} from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import {
	LineChart,
	Line,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	Legend,
	ResponsiveContainer,
	BarChart,
	Bar,
	PieChart,
	Pie,
	Cell
} from 'recharts';
import {
	TrendingUp,
	CloudDownload,
	CloudUpload,
	NetworkCheck,
	Schedule,
	Computer,
	Refresh
} from '@mui/icons-material';
import { invoke } from '@tauri-apps/api/core';
import dayjs from 'dayjs';

const WeeklyNetworkActivityPage = () => {
	const [startDate, setStartDate] = useState(dayjs().subtract(7, 'day'));
	const [endDate, setEndDate] = useState(dayjs());
	const [networkData, setNetworkData] = useState([]);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState(null);
	const [totalStats, setTotalStats] = useState({
		totalIncoming: 0,
		totalOutgoing: 0,
		totalDuration: 0,
		uniqueHosts: 0,
		uniqueServices: 0,
		totalSessions: 0
	});

	useEffect(() => {
		fetchNetworkData();
	}, []);

	const fetchNetworkData = async () => {
		setLoading(true);
		setError(null);
		
		try {
			const startDateStr = startDate.format('YYYY-MM-DD');
			const endDateStr = endDate.format('YYYY-MM-DD');
			
			const data = await invoke('get_network_history', {
				startDate: startDateStr,
				endDate: endDateStr
			});
			
			setNetworkData(data);
			calculateTotalStats(data);
		} catch (err) {
			setError(`Failed to fetch network data: ${err}`);
			console.error('Error fetching network data:', err);
		} finally {
			setLoading(false);
		}
	};

	const calculateTotalStats = (data) => {
		const stats = data.reduce((acc, day) => ({
			totalIncoming: acc.totalIncoming + day.total_incoming_bytes,
			totalOutgoing: acc.totalOutgoing + day.total_outgoing_bytes,
			totalDuration: acc.totalDuration + day.total_duration,
			uniqueHosts: Math.max(acc.uniqueHosts, day.unique_hosts),
			uniqueServices: Math.max(acc.uniqueServices, day.unique_services),
			totalSessions: acc.totalSessions + day.sessions.length
		}), {
			totalIncoming: 0,
			totalOutgoing: 0,
			totalDuration: 0,
			uniqueHosts: 0,
			uniqueServices: 0,
			totalSessions: 0
		});
		
		setTotalStats(stats);
	};

	const formatBytes = (bytes) => {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	};

	const formatDuration = (seconds) => {
		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const secs = seconds % 60;
		return `${hours}h ${minutes}m ${secs}s`;
	};

	// Prepare chart data
	const chartData = networkData.map(day => ({
		date: day.date,
		incoming: Math.round(day.total_incoming_bytes / (1024 * 1024)), // Convert to MB
		outgoing: Math.round(day.total_outgoing_bytes / (1024 * 1024)), // Convert to MB
		sessions: day.sessions.length,
		duration: Math.round(day.total_duration / 60), // Convert to minutes
		hosts: day.unique_hosts,
		services: day.unique_services
	}));

	// Prepare pie chart data
	const pieData = [
		{ name: 'Incoming', value: totalStats.totalIncoming, color: '#2196f3' },
		{ name: 'Outgoing', value: totalStats.totalOutgoing, color: '#ff9800' }
	];

	return (
		<LocalizationProvider dateAdapter={AdapterDayjs}>
			<Container maxWidth="xl" sx={{ py: 3 }}>
				<Typography variant="h4" gutterBottom>
					<NetworkCheck sx={{ mr: 1, verticalAlign: 'middle' }} />
					Weekly Network Activity
				</Typography>

				{/* Date Selection */}
				<Card sx={{ mb: 3 }}>
					<CardContent>
						<Grid container spacing={3} alignItems="center">
							<Grid item xs={12} md={3}>
								<DatePicker
									label="Start Date"
									value={startDate}
									onChange={(newValue) => setStartDate(newValue)}
									slotProps={{ textField: { fullWidth: true } }}
								/>
							</Grid>
							<Grid item xs={12} md={3}>
								<DatePicker
									label="End Date"
									value={endDate}
									onChange={(newValue) => setEndDate(newValue)}
									slotProps={{ textField: { fullWidth: true } }}
								/>
							</Grid>
							<Grid item xs={12} md={3}>
								<Button
									variant="contained"
									onClick={fetchNetworkData}
									disabled={loading}
									startIcon={loading ? <CircularProgress size={20} /> : <Refresh />}
									fullWidth
								>
									{loading ? 'Loading...' : 'Fetch Data'}
								</Button>
							</Grid>
						</Grid>
					</CardContent>
				</Card>

				{error && (
					<Alert severity="error" sx={{ mb: 3 }}>
						{error}
					</Alert>
				)}

				{/* Summary Statistics */}
				<Grid container spacing={3} sx={{ mb: 3 }}>
					<Grid item xs={12} sm={6} md={2}>
						<Card>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
									<CloudDownload sx={{ mr: 1, color: 'primary.main' }} />
									<Typography variant="h6" color="primary">
										Incoming
									</Typography>
								</Box>
								<Typography variant="h5">
									{formatBytes(totalStats.totalIncoming)}
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={2}>
						<Card>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
									<CloudUpload sx={{ mr: 1, color: 'secondary.main' }} />
									<Typography variant="h6" color="secondary">
										Outgoing
									</Typography>
								</Box>
								<Typography variant="h5">
									{formatBytes(totalStats.totalOutgoing)}
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={2}>
						<Card>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
									<Schedule sx={{ mr: 1, color: 'info.main' }} />
									<Typography variant="h6" color="info.main">
										Duration
									</Typography>
								</Box>
								<Typography variant="h5">
									{formatDuration(totalStats.totalDuration)}
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={2}>
						<Card>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
									<Computer sx={{ mr: 1, color: 'success.main' }} />
									<Typography variant="h6" color="success.main">
										Hosts
									</Typography>
								</Box>
								<Typography variant="h5">
									{totalStats.uniqueHosts}
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={2}>
						<Card>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
									<TrendingUp sx={{ mr: 1, color: 'warning.main' }} />
									<Typography variant="h6" color="warning.main">
										Services
									</Typography>
								</Box>
								<Typography variant="h5">
									{totalStats.uniqueServices}
								</Typography>
							</CardContent>
						</Card>
					</Grid>
					<Grid item xs={12} sm={6} md={2}>
						<Card>
							<CardContent>
								<Box sx={{ display: 'flex', alignItems: 'center', mb: 1 }}>
									<NetworkCheck sx={{ mr: 1, color: 'text.secondary' }} />
									<Typography variant="h6" color="text.secondary">
										Sessions
									</Typography>
								</Box>
								<Typography variant="h5">
									{totalStats.totalSessions}
								</Typography>
							</CardContent>
						</Card>
					</Grid>
				</Grid>

				{/* Charts */}
				<Grid container spacing={3}>
					{/* Daily Traffic Chart */}
					<Grid item xs={12} lg={8}>
						<Card>
							<CardContent>
								<Typography variant="h6" gutterBottom>
									Daily Network Traffic (MB)
								</Typography>
								<ResponsiveContainer width="100%" height={300}>
									<LineChart data={chartData}>
										<CartesianGrid strokeDasharray="3 3" />
										<XAxis dataKey="date" />
										<YAxis />
										<Tooltip formatter={(value) => [`${value} MB`, '']} />
										<Legend />
										<Line
											type="monotone"
											dataKey="incoming"
											stroke="#2196f3"
											strokeWidth={3}
											name="Incoming"
										/>
										<Line
											type="monotone"
											dataKey="outgoing"
											stroke="#ff9800"
											strokeWidth={3}
											name="Outgoing"
										/>
									</LineChart>
								</ResponsiveContainer>
							</CardContent>
						</Card>
					</Grid>

					{/* Traffic Distribution */}
					<Grid item xs={12} lg={4}>
						<Card>
							<CardContent>
								<Typography variant="h6" gutterBottom>
									Traffic Distribution
								</Typography>
								<ResponsiveContainer width="100%" height={300}>
									<PieChart>
										<Pie
											data={pieData}
											cx="50%"
											cy="50%"
											labelLine={false}
											label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
											outerRadius={80}
											fill="#8884d8"
											dataKey="value"
										>
											{pieData.map((entry, index) => (
												<Cell key={`cell-${index}`} fill={entry.color} />
											))}
										</Pie>
										<Tooltip formatter={(value) => formatBytes(value)} />
									</PieChart>
								</ResponsiveContainer>
							</CardContent>
						</Card>
					</Grid>

					{/* Daily Sessions */}
					<Grid item xs={12} lg={6}>
						<Card>
							<CardContent>
								<Typography variant="h6" gutterBottom>
									Daily Sessions & Duration
								</Typography>
								<ResponsiveContainer width="100%" height={300}>
									<BarChart data={chartData}>
										<CartesianGrid strokeDasharray="3 3" />
										<XAxis dataKey="date" />
										<YAxis yAxisId="left" />
										<YAxis yAxisId="right" orientation="right" />
										<Tooltip />
										<Legend />
										<Bar yAxisId="left" dataKey="sessions" fill="#4caf50" name="Sessions" />
										<Bar yAxisId="right" dataKey="duration" fill="#9c27b0" name="Duration (min)" />
									</BarChart>
								</ResponsiveContainer>
							</CardContent>
						</Card>
					</Grid>

					{/* Hosts & Services */}
					<Grid item xs={12} lg={6}>
						<Card>
							<CardContent>
								<Typography variant="h6" gutterBottom>
									Daily Hosts & Services
								</Typography>
								<ResponsiveContainer width="100%" height={300}>
									<BarChart data={chartData}>
										<CartesianGrid strokeDasharray="3 3" />
										<XAxis dataKey="date" />
										<YAxis />
										<Tooltip />
										<Legend />
										<Bar dataKey="hosts" fill="#2196f3" name="Unique Hosts" />
										<Bar dataKey="services" fill="#ff9800" name="Unique Services" />
									</BarChart>
								</ResponsiveContainer>
							</CardContent>
						</Card>
					</Grid>
				</Grid>

				{/* Daily Details */}
				{networkData.length > 0 && (
					<Card sx={{ mt: 3 }}>
						<CardContent>
							<Typography variant="h6" gutterBottom>
								Daily Details
							</Typography>
							<List>
								{networkData.map((day, index) => (
									<React.Fragment key={day.date}>
										<ListItem>
											<ListItemText
												primary={
													<Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
														<Typography variant="subtitle1" fontWeight="bold">
															{day.date}
														</Typography>
														<Box sx={{ display: 'flex', gap: 1 }}>
															<Chip 
																label={`${day.sessions.length} sessions`} 
																size="small" 
																color="primary"
																variant="outlined"
															/>
															<Chip 
																label={formatBytes(day.total_incoming_bytes + day.total_outgoing_bytes)} 
																size="small" 
																color="secondary"
																variant="outlined"
															/>
														</Box>
													</Box>
												}
												secondary={
													<Box sx={{ mt: 1 }}>
														<Typography variant="body2" color="text.secondary">
															↓ {formatBytes(day.total_incoming_bytes)} | 
															↑ {formatBytes(day.total_outgoing_bytes)} | 
															Duration: {formatDuration(day.total_duration)} | 
															Hosts: {day.unique_hosts} | 
															Services: {day.unique_services}
														</Typography>
													</Box>
												}
											/>
										</ListItem>
										{index < networkData.length - 1 && <Divider />}
									</React.Fragment>
								))}
							</List>
						</CardContent>
					</Card>
				)}
			</Container>
		</LocalizationProvider>
	);
};

export default WeeklyNetworkActivityPage;
