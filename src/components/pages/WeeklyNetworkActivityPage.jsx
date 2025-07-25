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
			<Container maxWidth="xl" sx={{ 
				py: { xs: 3, md: 4 }, 
				px: { xs: 2, md: 3 },
				background: 'linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)',
				minHeight: '100vh'
			}}>
				{/* Header */}
				<Paper elevation={0} sx={{ 
					mb: 4, 
					p: { xs: 3, md: 4 },
					background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
					color: 'white',
					borderRadius: 3
				}}>
					<Typography variant="h4" gutterBottom sx={{ 
						display: 'flex', 
						alignItems: 'center',
						fontSize: { xs: '1.75rem', md: '2.125rem' },
						fontWeight: 700,
						mb: 2
					}}>
						<NetworkCheck sx={{ mr: 2, fontSize: { xs: '2rem', md: '2.5rem' } }} />
						Weekly Network Activity
					</Typography>
					<Typography variant="h6" sx={{ 
						opacity: 0.95,
						fontSize: { xs: '1rem', md: '1.25rem' },
						fontWeight: 400
					}}>
						Comprehensive analysis of your network usage patterns and traffic statistics
					</Typography>
				</Paper>

				{/* Date Selection */}
				<Card sx={{ 
					mb: 4, 
					boxShadow: '0 8px 32px rgba(0,0,0,0.1)', 
					borderRadius: 3,
					border: '1px solid rgba(255,255,255,0.2)'
				}}>
					<CardContent sx={{ p: { xs: 3, md: 4 } }}>
						<Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
							<Schedule sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
							<Typography variant="h5" sx={{ 
								fontWeight: 600,
								color: 'text.primary'
							}}>
								Select Date Range
							</Typography>
						</Box>
						<Grid container spacing={3} alignItems="end">
							<Grid item xs={12} sm={6} md={3}>
								<DatePicker
									label="Start Date"
									value={startDate}
									onChange={(newValue) => setStartDate(newValue)}
									slotProps={{ 
										textField: { 
											fullWidth: true,
											size: 'medium',
											variant: 'outlined'
										} 
									}}
								/>
							</Grid>
							<Grid item xs={12} sm={6} md={3}>
								<DatePicker
									label="End Date"
									value={endDate}
									onChange={(newValue) => setEndDate(newValue)}
									slotProps={{ 
										textField: { 
											fullWidth: true,
											size: 'medium',
											variant: 'outlined'
										} 
									}}
								/>
							</Grid>
							<Grid item xs={12} sm={8} md={4}>
								<Button
									variant="contained"
									onClick={fetchNetworkData}
									disabled={loading}
									startIcon={loading ? <CircularProgress size={20} color="inherit" /> : <Refresh />}
									fullWidth
									size="large"
									sx={{ 
										py: 2,
										borderRadius: 2,
										textTransform: 'none',
										fontSize: '1rem',
										fontWeight: 600,
										background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
										'&:hover': {
											background: 'linear-gradient(45deg, #1976D2 30%, #1CB5E0 90%)',
										}
									}}
								>
									{loading ? 'Loading...' : 'Fetch Data'}
								</Button>
							</Grid>
							<Grid item xs={12} sm={4} md={2}>
								<Box sx={{ 
									textAlign: 'center',
									p: 2,
									bgcolor: 'action.hover',
									borderRadius: 2,
									border: '1px solid',
									borderColor: 'divider'
								}}>
									<Typography variant="body2" color="text.secondary" fontWeight={500}>
										{networkData.length > 0 ? `${networkData.length} days loaded` : 'No data'}
									</Typography>
								</Box>
							</Grid>
						</Grid>
					</CardContent>
				</Card>

				{error && (
					<Alert 
						severity="error" 
						sx={{ 
							mb: 4, 
							borderRadius: 3,
							boxShadow: '0 4px 20px rgba(244, 67, 54, 0.15)',
							border: '1px solid rgba(244, 67, 54, 0.2)'
						}}
					>
						<Typography variant="body1" fontWeight={500}>{error}</Typography>
					</Alert>
				)}

				{/* Summary Statistics */}
				<Card sx={{ 
					mb: 4,
					boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
					borderRadius: 3,
					border: '1px solid rgba(255,255,255,0.2)'
				}}>
					<CardContent sx={{ p: { xs: 3, md: 4 } }}>
						<Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
							<TrendingUp sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
							<Typography variant="h5" sx={{ fontWeight: 600, color: 'text.primary' }}>
								Summary Statistics
							</Typography>
						</Box>
						<Grid container spacing={3}>
							<Grid item xs={6} sm={4} md={2}>
								<Card sx={{ 
									height: '100%', 
									boxShadow: '0 4px 20px rgba(33, 150, 243, 0.15)',
									borderRadius: 2,
									border: '2px solid rgba(33, 150, 243, 0.1)',
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-4px)',
										boxShadow: '0 8px 25px rgba(33, 150, 243, 0.25)',
										borderColor: 'primary.main'
									}
								}}>
									<CardContent sx={{ p: 3, textAlign: 'center' }}>
										<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
											<CloudDownload sx={{ 
												fontSize: 48, 
												color: 'primary.main', 
												mb: 1.5 
											}} />
											<Typography variant="h6" color="primary" sx={{ 
												fontSize: '1rem',
												fontWeight: 700,
												textTransform: 'uppercase',
												letterSpacing: 1
											}}>
												Incoming
											</Typography>
										</Box>
										<Typography variant="h4" sx={{ 
											fontSize: { xs: '1.5rem', md: '1.75rem' },
											fontWeight: 800,
											color: 'text.primary'
										}}>
											{formatBytes(totalStats.totalIncoming)}
										</Typography>
									</CardContent>
								</Card>
							</Grid>
							<Grid item xs={6} sm={4} md={2}>
								<Card sx={{ 
									height: '100%', 
									boxShadow: '0 4px 20px rgba(255, 152, 0, 0.15)',
									borderRadius: 2,
									border: '2px solid rgba(255, 152, 0, 0.1)',
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-4px)',
										boxShadow: '0 8px 25px rgba(255, 152, 0, 0.25)',
										borderColor: 'secondary.main'
									}
								}}>
									<CardContent sx={{ p: 3, textAlign: 'center' }}>
										<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
											<CloudUpload sx={{ 
												fontSize: 48, 
												color: 'secondary.main', 
												mb: 1.5 
											}} />
											<Typography variant="h6" color="secondary" sx={{ 
												fontSize: '1rem',
												fontWeight: 700,
												textTransform: 'uppercase',
												letterSpacing: 1
											}}>
												Outgoing
											</Typography>
										</Box>
										<Typography variant="h4" sx={{ 
											fontSize: { xs: '1.5rem', md: '1.75rem' },
											fontWeight: 800,
											color: 'text.primary'
										}}>
											{formatBytes(totalStats.totalOutgoing)}
										</Typography>
									</CardContent>
								</Card>
							</Grid>
							<Grid item xs={6} sm={4} md={2}>
								<Card sx={{ 
									height: '100%', 
									boxShadow: '0 4px 20px rgba(33, 150, 243, 0.15)',
									borderRadius: 2,
									border: '2px solid rgba(33, 150, 243, 0.1)',
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-4px)',
										boxShadow: '0 8px 25px rgba(33, 150, 243, 0.25)',
										borderColor: 'info.main'
									}
								}}>
									<CardContent sx={{ p: 3, textAlign: 'center' }}>
										<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
											<Schedule sx={{ 
												fontSize: 48, 
												color: 'info.main', 
												mb: 1.5 
											}} />
											<Typography variant="h6" color="info.main" sx={{ 
												fontSize: '1rem',
												fontWeight: 700,
												textTransform: 'uppercase',
												letterSpacing: 1
											}}>
												Duration
											</Typography>
										</Box>
										<Typography variant="h4" sx={{ 
											fontSize: { xs: '1.5rem', md: '1.75rem' },
											fontWeight: 800,
											color: 'text.primary'
										}}>
											{formatDuration(totalStats.totalDuration)}
										</Typography>
									</CardContent>
								</Card>
							</Grid>
							<Grid item xs={6} sm={4} md={2}>
								<Card sx={{ 
									height: '100%', 
									boxShadow: '0 4px 20px rgba(76, 175, 80, 0.15)',
									borderRadius: 2,
									border: '2px solid rgba(76, 175, 80, 0.1)',
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-4px)',
										boxShadow: '0 8px 25px rgba(76, 175, 80, 0.25)',
										borderColor: 'success.main'
									}
								}}>
									<CardContent sx={{ p: 3, textAlign: 'center' }}>
										<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
											<Computer sx={{ 
												fontSize: 48, 
												color: 'success.main', 
												mb: 1.5 
											}} />
											<Typography variant="h6" color="success.main" sx={{ 
												fontSize: '1rem',
												fontWeight: 700,
												textTransform: 'uppercase',
												letterSpacing: 1
											}}>
												Hosts
											</Typography>
										</Box>
										<Typography variant="h4" sx={{ 
											fontSize: { xs: '1.5rem', md: '1.75rem' },
											fontWeight: 800,
											color: 'text.primary'
										}}>
											{totalStats.uniqueHosts}
										</Typography>
									</CardContent>
								</Card>
							</Grid>
							<Grid item xs={6} sm={4} md={2}>
								<Card sx={{ 
									height: '100%', 
									boxShadow: '0 4px 20px rgba(255, 152, 0, 0.15)',
									borderRadius: 2,
									border: '2px solid rgba(255, 152, 0, 0.1)',
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-4px)',
										boxShadow: '0 8px 25px rgba(255, 152, 0, 0.25)',
										borderColor: 'warning.main'
									}
								}}>
									<CardContent sx={{ p: 3, textAlign: 'center' }}>
										<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
											<TrendingUp sx={{ 
												fontSize: 48, 
												color: 'warning.main', 
												mb: 1.5 
											}} />
											<Typography variant="h6" color="warning.main" sx={{ 
												fontSize: '1rem',
												fontWeight: 700,
												textTransform: 'uppercase',
												letterSpacing: 1
											}}>
												Services
											</Typography>
										</Box>
										<Typography variant="h4" sx={{ 
											fontSize: { xs: '1.5rem', md: '1.75rem' },
											fontWeight: 800,
											color: 'text.primary'
										}}>
											{totalStats.uniqueServices}
										</Typography>
									</CardContent>
								</Card>
							</Grid>
							<Grid item xs={6} sm={4} md={2}>
								<Card sx={{ 
									height: '100%', 
									boxShadow: '0 4px 20px rgba(156, 39, 176, 0.15)',
									borderRadius: 2,
									border: '2px solid rgba(156, 39, 176, 0.1)',
									transition: 'all 0.3s ease',
									'&:hover': {
										transform: 'translateY(-4px)',
										boxShadow: '0 8px 25px rgba(156, 39, 176, 0.25)',
										borderColor: 'purple'
									}
								}}>
									<CardContent sx={{ p: 3, textAlign: 'center' }}>
										<Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mb: 2 }}>
											<NetworkCheck sx={{ 
												fontSize: 48, 
												color: '#9c27b0', 
												mb: 1.5 
											}} />
											<Typography variant="h6" sx={{ 
												fontSize: '1rem',
												fontWeight: 700,
												textTransform: 'uppercase',
												letterSpacing: 1,
												color: '#9c27b0'
											}}>
												Sessions
											</Typography>
										</Box>
										<Typography variant="h4" sx={{ 
											fontSize: { xs: '1.5rem', md: '1.75rem' },
											fontWeight: 800,
											color: 'text.primary'
										}}>
											{totalStats.totalSessions}
										</Typography>
									</CardContent>
								</Card>
							</Grid>
						</Grid>
					</CardContent>
				</Card>
				{/* Charts */}
				<Card sx={{ 
					mb: 4,
					boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
					borderRadius: 3,
					border: '1px solid rgba(255,255,255,0.2)'
				}}>
					<CardContent sx={{ p: { xs: 3, md: 4 } }}>
						<Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
							<TrendingUp sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
							<Typography variant="h5" sx={{ fontWeight: 600, color: 'text.primary' }}>
								Analytics & Trends
							</Typography>
						</Box>
						{/* First Row - 2 Charts */}
						<Grid container spacing={0} sx={{ mb: 0 }}>
							{/* Daily Traffic Chart */}
							<Grid item xs={12} md={6} sx={{ pr: { md: 1 } }}>
								<Card sx={{ 
									height: { xs: 400, md: 450 }, 
									boxShadow: '0 4px 20px rgba(0,0,0,0.08)',
									borderRadius: 2,
									border: '1px solid rgba(0,0,0,0.08)',
									display: 'flex',
									flexDirection: 'column',
									m: 0
								}}>
									<CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column', width: '100%' }}>
										<Typography variant="h6" gutterBottom sx={{ fontWeight: 600, mb: 2, fontSize: { xs: '1rem', md: '1.25rem' } }}>
											Daily Network Traffic (MB)
										</Typography>
										<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
											<ResponsiveContainer width="100%" height="100%">
												<LineChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 20 }}>
													<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
													<XAxis 
														dataKey="date" 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<YAxis 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<Tooltip 
														formatter={(value) => [`${value} MB`, '']} 
														contentStyle={{
															backgroundColor: '#fff',
															border: '1px solid #ddd',
															borderRadius: 8,
															boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
														}}
													/>
													<Legend />
													<Line
														type="monotone"
														dataKey="incoming"
														stroke="#2196f3"
														strokeWidth={3}
														name="Incoming"
														dot={{ fill: '#2196f3', strokeWidth: 2, r: 4 }}
														activeDot={{ r: 6, stroke: '#2196f3', strokeWidth: 2 }}
													/>
													<Line
														type="monotone"
														dataKey="outgoing"
														stroke="#ff9800"
														strokeWidth={3}
														name="Outgoing"
														dot={{ fill: '#ff9800', strokeWidth: 2, r: 4 }}
														activeDot={{ r: 6, stroke: '#ff9800', strokeWidth: 2 }}
													/>
												</LineChart>
											</ResponsiveContainer>
										</Box>
									</CardContent>
								</Card>
							</Grid>

							{/* Traffic Distribution */}
							<Grid item xs={12} md={6} sx={{ pl: { md: 1 } }}>
								<Card sx={{ 
									height: { xs: 400, md: 450 }, 
									boxShadow: '0 4px 20px rgba(0,0,0,0.08)',
									borderRadius: 2,
									border: '1px solid rgba(0,0,0,0.08)',
									display: 'flex',
									flexDirection: 'column',
									m: 0
								}}>
									<CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column' }}>
										<Typography variant="h6" gutterBottom sx={{ fontWeight: 600, mb: 2, fontSize: { xs: '1rem', md: '1.25rem' } }}>
											Traffic Distribution
										</Typography>
										<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
											<ResponsiveContainer width="100%" height="100%">
												<PieChart>
													<Pie
														data={pieData}
														cx="50%"
														cy="50%"
														labelLine={false}
														label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
														outerRadius={120}
														fill="#8884d8"
														dataKey="value"
													>
														{pieData.map((entry, index) => (
															<Cell key={`cell-${index}`} fill={entry.color} />
														))}
													</Pie>
													<Tooltip 
														formatter={(value) => formatBytes(value)}
														contentStyle={{
															backgroundColor: '#fff',
															border: '1px solid #ddd',
															borderRadius: 8,
															boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
														}}
													/>
												</PieChart>
											</ResponsiveContainer>
										</Box>
									</CardContent>
								</Card>
							</Grid>
						</Grid>

						{/* Second Row - 2 Charts */}
						<Grid container spacing={0} sx={{ mt: 2 }}>
							{/* Daily Sessions */}
							<Grid item xs={12} md={6} sx={{ pr: { md: 1 } }}>
								<Card sx={{ 
									height: { xs: 400, md: 450 }, 
									boxShadow: '0 4px 20px rgba(0,0,0,0.08)',
									borderRadius: 2,
									border: '1px solid rgba(0,0,0,0.08)',
									display: 'flex',
									flexDirection: 'column',
									m: 0
								}}>
									<CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column' }}>
										<Typography variant="h6" gutterBottom sx={{ fontWeight: 600, mb: 2, fontSize: { xs: '1rem', md: '1.25rem' } }}>
											Daily Sessions & Duration
										</Typography>
										<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
											<ResponsiveContainer width="100%" height="100%">
												<BarChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 20 }}>
													<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
													<XAxis 
														dataKey="date" 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<YAxis 
														yAxisId="left" 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<YAxis 
														yAxisId="right" 
														orientation="right" 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<Tooltip 
														contentStyle={{
															backgroundColor: '#fff',
															border: '1px solid #ddd',
															borderRadius: 8,
															boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
														}}
													/>
													<Legend />
													<Bar yAxisId="left" dataKey="sessions" fill="#4caf50" name="Sessions" radius={[4, 4, 0, 0]} />
													<Bar yAxisId="right" dataKey="duration" fill="#9c27b0" name="Duration (min)" radius={[4, 4, 0, 0]} />
												</BarChart>
											</ResponsiveContainer>
										</Box>
									</CardContent>
								</Card>
							</Grid>

							{/* Hosts & Services */}
							<Grid item xs={12} md={6} sx={{ pl: { md: 1 } }}>
								<Card sx={{ 
									height: { xs: 400, md: 450 }, 
									boxShadow: '0 4px 20px rgba(0,0,0,0.08)',
									borderRadius: 2,
									border: '1px solid rgba(0,0,0,0.08)',
									display: 'flex',
									flexDirection: 'column',
									m: 0
								}}>
									<CardContent sx={{ p: 3, height: '100%', display: 'flex', flexDirection: 'column' }}>
										<Typography variant="h6" gutterBottom sx={{ fontWeight: 600, mb: 2, fontSize: { xs: '1rem', md: '1.25rem' } }}>
											Daily Hosts & Services
										</Typography>
										<Box sx={{ height: { xs: 300, md: 350 }, flex: 1 }}>
											<ResponsiveContainer width="100%" height="100%">
												<BarChart data={chartData} margin={{ top: 20, right: 30, left: 20, bottom: 20 }}>
													<CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
													<XAxis 
														dataKey="date" 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<YAxis 
														fontSize={12}
														tick={{ fill: '#666' }}
														axisLine={{ stroke: '#ddd' }}
													/>
													<Tooltip 
														contentStyle={{
															backgroundColor: '#fff',
															border: '1px solid #ddd',
															borderRadius: 8,
															boxShadow: '0 4px 6px rgba(0,0,0,0.1)'
														}}
													/>
													<Legend />
													<Bar dataKey="hosts" fill="#2196f3" name="Unique Hosts" radius={[4, 4, 0, 0]} />
													<Bar dataKey="services" fill="#ff9800" name="Unique Services" radius={[4, 4, 0, 0]} />
												</BarChart>
											</ResponsiveContainer>
										</Box>
									</CardContent>
								</Card>
							</Grid>
						</Grid>
					</CardContent>
				</Card>

				{/* Daily Details */}
				{networkData.length > 0 && (
					<Card sx={{ 
						boxShadow: '0 8px 32px rgba(0,0,0,0.1)',
						borderRadius: 3,
						border: '1px solid rgba(255,255,255,0.2)'
					}}>
						<CardContent sx={{ p: { xs: 3, md: 4 } }}>
							<Box sx={{ display: 'flex', alignItems: 'center', mb: 4 }}>
								<Computer sx={{ mr: 2, color: 'primary.main', fontSize: '1.5rem' }} />
								<Typography variant="h5" sx={{ fontWeight: 600, color: 'text.primary' }}>
									Daily Details
								</Typography>
							</Box>
							<Box sx={{ 
								maxHeight: { xs: 450, md: 550 }, 
								overflow: 'auto',
								bgcolor: 'grey.50',
								borderRadius: 2,
								p: 1,
								'&::-webkit-scrollbar': {
									width: '8px',
								},
								'&::-webkit-scrollbar-track': {
									background: '#f1f1f1',
									borderRadius: '4px',
								},
								'&::-webkit-scrollbar-thumb': {
									background: 'linear-gradient(45deg, #2196F3 30%, #21CBF3 90%)',
									borderRadius: '4px',
								},
								'&::-webkit-scrollbar-thumb:hover': {
									background: 'linear-gradient(45deg, #1976D2 30%, #1CB5E0 90%)',
								},
							}}>
								<List sx={{ p: 0 }}>
									{networkData.map((day, index) => (
										<React.Fragment key={day.date}>
											<Card sx={{
												mb: 2,
												transition: 'all 0.3s ease',
												'&:hover': {
													transform: 'translateY(-2px)',
													boxShadow: '0 8px 25px rgba(0,0,0,0.1)'
												}
											}}>
												<ListItem sx={{ 
													py: 3,
													px: 3
												}}>
													<ListItemText
														primary={
															<Box sx={{ 
																display: 'flex', 
																justifyContent: 'space-between', 
																alignItems: { xs: 'flex-start', sm: 'center' },
																flexDirection: { xs: 'column', sm: 'row' },
																gap: { xs: 2, sm: 0 },
																mb: 2
															}}>
																<Typography variant="h6" fontWeight="bold" sx={{
																	fontSize: { xs: '1.1rem', md: '1.25rem' },
																	color: 'primary.main'
																}}>
																	{dayjs(day.date).format('dddd, MMMM D, YYYY')}
																</Typography>
																<Box sx={{ 
																	display: 'flex', 
																	gap: 1.5, 
																	flexWrap: 'wrap',
																	alignItems: 'center'
																}}>
																	<Chip 
																		label={`${day.sessions.length} session${day.sessions.length !== 1 ? 's' : ''}`} 
																		size="medium" 
																		color="primary"
																		variant="filled"
																		sx={{ 
																			fontWeight: 600,
																			borderRadius: 3
																		}}
																	/>
																	<Chip 
																		label={formatBytes(day.total_incoming_bytes + day.total_outgoing_bytes)} 
																		size="medium" 
																		color="secondary"
																		variant="filled"
																		sx={{ 
																			fontWeight: 600,
																			borderRadius: 3
																		}}
																	/>
																	{day.total_duration > 0 && (
																		<Chip 
																			label={formatDuration(day.total_duration)} 
																			size="medium" 
																			color="info"
																			variant="filled"
																			sx={{ 
																				fontWeight: 600,
																				borderRadius: 3
																			}}
																		/>
																	)}
																</Box>
															</Box>
														}
														secondary={
															<Box sx={{ mt: 2 }}>
																<Grid container spacing={2}>
																	<Grid item xs={6} sm={3}>
																		<Box sx={{ 
																			display: 'flex', 
																			alignItems: 'center', 
																			gap: 1.5,
																			p: 2,
																			bgcolor: 'primary.50',
																			borderRadius: 2,
																			border: '1px solid',
																			borderColor: 'primary.200'
																		}}>
																			<CloudDownload sx={{ fontSize: 20, color: 'primary.main' }} />
																			<Box>
																				<Typography variant="caption" color="text.secondary" sx={{
																					fontSize: '0.75rem',
																					fontWeight: 500
																				}}>
																					Downloaded
																				</Typography>
																				<Typography variant="body2" fontWeight="bold" sx={{
																					fontSize: { xs: '0.875rem', md: '1rem' },
																					color: 'primary.main'
																				}}>
																					{formatBytes(day.total_incoming_bytes)}
																				</Typography>
																			</Box>
																		</Box>
																	</Grid>
																	<Grid item xs={6} sm={3}>
																		<Box sx={{ 
																			display: 'flex', 
																			alignItems: 'center', 
																			gap: 1.5,
																			p: 2,
																			bgcolor: 'secondary.50',
																			borderRadius: 2,
																			border: '1px solid',
																			borderColor: 'secondary.200'
																		}}>
																			<CloudUpload sx={{ fontSize: 20, color: 'secondary.main' }} />
																			<Box>
																				<Typography variant="caption" color="text.secondary" sx={{
																					fontSize: '0.75rem',
																					fontWeight: 500
																				}}>
																					Uploaded
																				</Typography>
																				<Typography variant="body2" fontWeight="bold" sx={{
																					fontSize: { xs: '0.875rem', md: '1rem' },
																					color: 'secondary.main'
																				}}>
																					{formatBytes(day.total_outgoing_bytes)}
																				</Typography>
																			</Box>
																		</Box>
																	</Grid>
																	<Grid item xs={6} sm={3}>
																		<Box sx={{ 
																			display: 'flex', 
																			alignItems: 'center', 
																			gap: 1.5,
																			p: 2,
																			bgcolor: 'success.50',
																			borderRadius: 2,
																			border: '1px solid',
																			borderColor: 'success.200'
																		}}>
																			<Computer sx={{ fontSize: 20, color: 'success.main' }} />
																			<Box>
																				<Typography variant="caption" color="text.secondary" sx={{
																					fontSize: '0.75rem',
																					fontWeight: 500
																				}}>
																					Hosts
																				</Typography>
																				<Typography variant="body2" fontWeight="bold" sx={{
																					fontSize: { xs: '0.875rem', md: '1rem' },
																					color: 'success.main'
																				}}>
																					{day.unique_hosts}
																				</Typography>
																			</Box>
																		</Box>
																	</Grid>
																	<Grid item xs={6} sm={3}>
																		<Box sx={{ 
																			display: 'flex', 
																			alignItems: 'center', 
																			gap: 1.5,
																			p: 2,
																			bgcolor: 'warning.50',
																			borderRadius: 2,
																			border: '1px solid',
																			borderColor: 'warning.200'
																		}}>
																			<TrendingUp sx={{ fontSize: 20, color: 'warning.main' }} />
																			<Box>
																				<Typography variant="caption" color="text.secondary" sx={{
																					fontSize: '0.75rem',
																					fontWeight: 500
																				}}>
																					Services
																				</Typography>
																				<Typography variant="body2" fontWeight="bold" sx={{
																					fontSize: { xs: '0.875rem', md: '1rem' },
																					color: 'warning.main'
																				}}>
																					{day.unique_services}
																				</Typography>
																			</Box>
																		</Box>
																	</Grid>
																</Grid>
															</Box>
														}
													/>
												</ListItem>
											</Card>
										</React.Fragment>
									))}
								</List>
							</Box>
						</CardContent>
					</Card>
				)}
			</Container>
		</LocalizationProvider>
	);
};

export default WeeklyNetworkActivityPage;
