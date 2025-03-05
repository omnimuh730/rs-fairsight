import React from 'react';
import { History, ShowChart, Scanner, DataObject, Language, Memory, Gesture, Insights, School, AccountCircle, PrivacyTip, Settings, PrecisionManufacturing, Mail, Factory, Storage, SimCard, Visibility, AutoMode } from '@mui/icons-material';

import Dashboard from '../pages/Dashboard';

export const pageRoutes = {
	'/': Dashboard,
	'/crawler': Dashboard,
};

export const NAVIGATION = [
	{
		kind: 'header',
		title: 'Demo',
	},
	{
		segment: 'crawler',
		title: 'Crawling',
		icon: Scanner,
	}
];