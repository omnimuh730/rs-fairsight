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
	try {
		const codePoints = countryCode
			.toUpperCase()
			.split('')
			.map(char => 127397 + char.charCodeAt());
		return String.fromCodePoint(...codePoints);
	} catch {
		return '🌐';
	}
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
	}
	
	// If name is too long, truncate it
	if (shortName.length > 10) {
		return shortName.substring(0, 8) + '..';
	}
	
	return shortName || 'Adapter';
};
