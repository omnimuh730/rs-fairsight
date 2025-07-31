import * as React from 'react';
import PropTypes from 'prop-types';
import { BrowserRouter as Router, Route, Routes, Link, Navigate } from 'react-router-dom';

import {
    AppBar, Box, Divider, Drawer, IconButton,
    List, ListItem, ListItemButton, ListItemIcon, ListItemText,
    Toolbar, Typography
} from '@mui/material';

import CssBaseline from '@mui/material/CssBaseline';

import {
    Timer,
    Timeline,
    AutoGraph,
    Monitor,
    NetworkCheck,
    TrendingUp,
    Menu
} from '@mui/icons-material'

import ShowTodayCard from '../ShowTodayPage/ShowToday';
import WeeklyActivityPage from '../WeeklyActivityPage';
import ChangeLogsPage from '../ChangeLogsPage';
import MonitorPage from '../MonitorPage';
import NetworkMonitorPage from '../NetworkMonitorPage';
import WeeklyNetworkActivityPage from '../WeeklyNetworkActivityPage';

const drawerWidth = 240;

// Define sidebar list items with text, icon, and route
const listItems = [
  { text: `Today's work`, icon: <Timer />, path: '/inbox' },
  { text: 'Weekly Activity', icon: <Timeline />, path: '/starred' },
  { text: 'Changelogs', icon: <AutoGraph />, path: '/sent' },
  { text: 'System Monitor', icon: <Monitor />, path: '/monitor' },
  { text: 'Traffic Monitor', icon: <NetworkCheck />, path: '/network' },
  { text: 'Network Activity', icon: <TrendingUp />, path: '/network-weekly' },
];

function ResponsiveDrawer(props) {
  const { window } = props;
  const [mobileOpen, setMobileOpen] = React.useState(false);
  const [isClosing, setIsClosing] = React.useState(false);

  const handleDrawerClose = () => {
    setIsClosing(true);
    setMobileOpen(false);
  };

  const handleDrawerTransitionEnd = () => {
    setIsClosing(false);
  };

  const handleDrawerToggle = () => {
    if (!isClosing) {
      setMobileOpen(!mobileOpen);
    }
  };

  const drawer = (
    <div>
      <Toolbar />
      <Divider />
      <List>
        {listItems.map((item) => (
          <ListItem key={item.text} disablePadding>
            <ListItemButton component={Link} to={item.path} onClick={handleDrawerClose}>
              <ListItemIcon>{item.icon}</ListItemIcon>
              <ListItemText primary={item.text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </div>
  );

  const container = window !== undefined ? () => window().document.body : undefined;

  return (
    <Box sx={{ display: 'flex' }}>
      <CssBaseline />
      <AppBar
        position="fixed"
        sx={{
          width: { sm: `calc(100% - ${drawerWidth}px)` },
          ml: { sm: `${drawerWidth}px` },
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { sm: 'none' } }}
          >
            <Menu/>
          </IconButton>
          <Typography variant="h6" noWrap component="div">
            Time is your ally, TinkerTicker is your guide.
          </Typography>
        </Toolbar>
      </AppBar>
      <Box
        component="nav"
        sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
        aria-label="mailbox folders"
      >
        <Drawer
          container={container}
          variant="temporary"
          open={mobileOpen}
          onTransitionEnd={handleDrawerTransitionEnd}
          onClose={handleDrawerClose}
          sx={{
            display: { xs: 'block', sm: 'none' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
          slotProps={{
            root: {
              keepMounted: true, // Better open performance on mobile.
            },
          }}
        >
          {drawer}
        </Drawer>
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: 'none', sm: 'block' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
          open
        >
          {drawer}
        </Drawer>
      </Box>
      <Box
        component="main"
        sx={{ flexGrow: 1, p: 3, width: { sm: `calc(100% - ${drawerWidth}px)` } }}
      >
        <Toolbar />
        <Box sx = {{ minWidth: 600}}>
        <Routes>
          <Route path="/inbox" element={<ShowTodayCard />} />
          <Route path="/starred" element={<WeeklyActivityPage />} />
          <Route path="/sent" element={<ChangeLogsPage />} />
          <Route path="/monitor" element={<MonitorPage />} />
          <Route path="/network" element={<NetworkMonitorPage />} />
          <Route path="/network-weekly" element={<WeeklyNetworkActivityPage />} />
          <Route path="*" element={<Navigate to="/inbox" />} /> {/* Default route */}
        </Routes>
        </Box>
      </Box>
    </Box>
  );
}

ResponsiveDrawer.propTypes = {
  window: PropTypes.func,
};

// Wrap the component with Router for it to work
export default function App() {
  return (
    <Router>
      <ResponsiveDrawer />
    </Router>
  );
}