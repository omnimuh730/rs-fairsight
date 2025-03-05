import * as React from 'react';
import PropTypes from 'prop-types';
import { BrowserRouter as Router, Route, Routes, Link, Navigate } from 'react-router-dom';

import {
    AppBar, Box, Divider, Drawer, IconButton,
    List, ListItem, ListItemButton, ListItemIcon, ListItemText,
    Toolbar, Typography, MenuIcon
} from '@mui/material';

import CssBaseline from '@mui/material/CssBaseline';

import {
    Timer,
    Timeline,
    AutoGraph,
    Feedback
} from '@mui/icons-material'

import ShowTodayCard from '../card/ShowToday';

const drawerWidth = 240;

// Define sidebar list items with text, icon, and route
const listItems = [
  { text: `Today's work`, icon: <Timer />, path: '/inbox' },
  { text: 'Weekly Activity', icon: <Timeline />, path: '/starred' },
  { text: 'Analytics', icon: <AutoGraph />, path: '/sent' },
  { text: 'Feedback', icon: <Feedback />, path: '/drafts' },
];

// Example page components
const InboxPage = () => (
  <Box>
    <ShowTodayCard/>
  </Box>
);

const StarredPage = () => (
  <Box>
    <Typography variant="h4">Starred</Typography>
    <Typography sx={{ marginBottom: 2 }}>
      These are your starred items.
    </Typography>
  </Box>
);

const SentPage = () => (
  <Box>
    <Typography variant="h4">Sent</Typography>
    <Typography sx={{ marginBottom: 2 }}>
      These are your sent messages.
    </Typography>
  </Box>
);

const DraftsPage = () => (
  <Box>
    <Typography variant="h4">Drafts</Typography>
    <Typography sx={{ marginBottom: 2 }}>
      Your draft messages are saved here.
    </Typography>
  </Box>
);

const NotFoundPage = () => (
  <Box>
    <Typography variant="h4">404 - Page Not Found</Typography>
    <Typography sx={{ marginBottom: 2 }}>
      Sorry, the page you are looking for does not exist.
    </Typography>
  </Box>
);

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
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" noWrap component="div">
            Project Fairsight - o1
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
        <Routes>
          <Route path="/inbox" element={<InboxPage />} />
          <Route path="/starred" element={<StarredPage />} />
          <Route path="/sent" element={<SentPage />} />
          <Route path="/drafts" element={<DraftsPage />} />
          <Route path="/" element={<Navigate to="/inbox" />} /> {/* Default route */}
          <Route path="*" element={<NotFoundPage />} /> {/* 404 route */}
        </Routes>
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