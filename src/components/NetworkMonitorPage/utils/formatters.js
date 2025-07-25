import React from 'react';

export const formatBytes = (bytes) => {
	if (bytes === 0) return '0 B';
	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

export const formatDuration = (seconds) => {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;
	return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
};

export const getCountryFlag = (countryCode) => {
	if (!countryCode) return '🌐';
	
	// For now, let's use flag emojis which work reliably
	const flagEmojis = {
		'US': '🇺🇸', 'CA': '🇨🇦', 'GB': '🇬🇧', 'DE': '🇩🇪', 'FR': '🇫🇷',
		'JP': '🇯🇵', 'CN': '🇨🇳', 'AU': '🇦🇺', 'BR': '🇧🇷', 'IN': '🇮🇳',
		'RU': '🇷🇺', 'IT': '🇮🇹', 'ES': '🇪🇸', 'NL': '🇳🇱', 'SE': '🇸🇪',
		'NO': '🇳🇴', 'DK': '🇩🇰', 'FI': '🇫🇮', 'CH': '🇨🇭', 'AT': '🇦🇹',
		'BE': '🇧🇪', 'IE': '🇮🇪', 'PT': '🇵🇹', 'PL': '🇵🇱', 'CZ': '🇨🇿',
		'HU': '🇭🇺', 'GR': '🇬🇷', 'TR': '🇹🇷', 'IL': '🇮🇱', 'ZA': '🇿🇦',
		'EG': '🇪🇬', 'AE': '🇦🇪', 'SA': '🇸🇦', 'KR': '🇰🇷', 'TH': '🇹🇭',
		'SG': '🇸🇬', 'MY': '🇲🇾', 'ID': '🇮🇩', 'PH': '🇵🇭', 'VN': '🇻🇳',
		'MX': '🇲🇽', 'AR': '🇦🇷', 'CL': '🇨🇱', 'CO': '🇨🇴', 'PE': '🇵🇪',
		'VE': '🇻🇪', 'UY': '🇺🇾', 'NZ': '🇳🇿', 'SK': '🇸🇰', 'SI': '🇸�',
		'HR': '🇭🇷', 'NG': '🇳🇬', 'KE': '🇰🇪'
	};
	
	return flagEmojis[countryCode?.toUpperCase()] || '🌐';
};

export const formatDomain = (hostname, domain) => {
	if (domain && domain !== hostname) {
		return domain;
	}
	
	if (hostname) {
		// Extract domain from hostname if not provided separately
		const parts = hostname.split('.');
		if (parts.length >= 2) {
			return `${parts[parts.length - 2]}.${parts[parts.length - 1]}`;
		}
		return hostname;
	}
	
	return null;
};

export const formatASN = (asn) => {
	if (!asn) return null;
	
	// If ASN contains organization info, extract just the AS number and org name
	if (asn.includes(' ')) {
		const parts = asn.split(' ');
		const asNumber = parts[0];
		const orgName = parts.slice(1).join(' ');
		
		// Truncate org name if too long
		if (orgName.length > 20) {
			return `${asNumber} ${orgName.substring(0, 17)}...`;
		}
		return asn;
	}
	
	return asn;
};

export const getHostTypeIcon = (ip, hostname, domain) => {
	// Determine what type of host this is based on available info
	if (domain) {
		if (domain.includes('google.com') || domain.includes('googleapis.com')) return '🔍';
		if (domain.includes('facebook.com') || domain.includes('meta.com')) return '📘';
		if (domain.includes('microsoft.com') || domain.includes('windows.com')) return '🪟';
		if (domain.includes('amazon.com') || domain.includes('amazonaws.com')) return '📦';
		if (domain.includes('cloudflare.com')) return '☁️';
		if (domain.includes('cdn')) return '🚀';
		return '🌐';
	}
	
	if (hostname) {
		return '🖥️';
	}
	
	// Check if it's a local IP
	if (ip.startsWith('192.168.') || ip.startsWith('10.') || ip.startsWith('172.')) {
		return '🏠';
	}
	
	return '🌍';
};

export const getCountryName = (countryCode) => {
	const countries = {
		'US': 'United States',
		'CA': 'Canada',
		'GB': 'United Kingdom',
		'DE': 'Germany',
		'FR': 'France',
		'JP': 'Japan',
		'AU': 'Australia',
		'CN': 'China',
		'IN': 'India',
		'BR': 'Brazil',
		'RU': 'Russia',
		'KR': 'South Korea',
		'NL': 'Netherlands',
		'SE': 'Sweden',
		'NO': 'Norway',
		'DK': 'Denmark',
		'FI': 'Finland',
		'CH': 'Switzerland',
		'AT': 'Austria',
		'BE': 'Belgium',
		'ES': 'Spain',
		'IT': 'Italy',
		'PT': 'Portugal',
		'IE': 'Ireland',
		'PL': 'Poland',
		'CZ': 'Czech Republic',
		'HU': 'Hungary',
		'SK': 'Slovakia',
		'SI': 'Slovenia',
		'HR': 'Croatia',
		'GR': 'Greece',
		'TR': 'Turkey',
		'IL': 'Israel',
		'AE': 'UAE',
		'SA': 'Saudi Arabia',
		'EG': 'Egypt',
		'ZA': 'South Africa',
		'NG': 'Nigeria',
		'KE': 'Kenya',
		'MX': 'Mexico',
		'AR': 'Argentina',
		'CL': 'Chile',
		'CO': 'Colombia',
		'PE': 'Peru',
		'TH': 'Thailand',
		'VN': 'Vietnam',
		'ID': 'Indonesia',
		'MY': 'Malaysia',
		'SG': 'Singapore',
		'PH': 'Philippines',
		'NZ': 'New Zealand'
	};
	
	return countries[countryCode?.toUpperCase()] || countryCode;
};

export const getShortAdapterName = (adapter) => {
	// Extract a short name from the adapter name
	let shortName = adapter.name;
	
	// Remove common prefixes and suffixes
	shortName = shortName.replace(/\\Device\\NPF_/, '');
	shortName = shortName.replace(/^{.*?}$/, 'Adapter');
	
	// For common adapter types, use friendly names
	if (adapter.description) {
		if (adapter.description.toLowerCase().includes('ethernet')) {
			return 'Ethernet';
		}
		if (adapter.description.toLowerCase().includes('wifi') || adapter.description.toLowerCase().includes('wireless')) {
			return 'WiFi';
		}
		if (adapter.description.toLowerCase().includes('vmware')) {
			return 'VMware';
		}
		if (adapter.description.toLowerCase().includes('loopback')) {
			return 'Loopback';
		}
		if (adapter.description.toLowerCase().includes('wan miniport')) {
			if (adapter.description.includes('IPv6')) return 'WAN6';
			if (adapter.description.includes('IP')) return 'WAN';
			return 'WAN';
		}
		if (adapter.description.toLowerCase().includes('openvpn') || adapter.description.toLowerCase().includes('tap')) {
			return 'VPN';
		}
	}
	
	// If name is too long, truncate it
	if (shortName.length > 10) {
		return shortName.substring(0, 8) + '..';
	}
	
	return shortName || 'Adapter';
};
