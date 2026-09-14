export function formatTime(timestamp: Date | number) {
	if (typeof timestamp === 'number') {
		timestamp = new Date(timestamp);
	}
	const hours = timestamp.getHours().toString().padStart(2, '0');
	const minutes = timestamp.getMinutes().toString().padStart(2, '0');
	return `${hours}:${minutes}`;
}

export function formatDuration(millis: number) {
	let seconds = Math.floor(millis / 1000) % 60;
	let minutes = Math.floor(millis / (60 * 1000)) % 60;
	let hours = Math.floor(millis / (60 * 60 * 1000)) % 24;
	if (hours >= 10) return `${hours}h`;
	else if (hours) return `${hours}h${minutes.toString()}m`;
	else if (minutes >= 10) return `${minutes}m`;
	else if (minutes) return `${minutes}m${seconds.toString()}s`;
	else return `${seconds}s`;
}

export function toMillis(hours: number, minutes: number, seconds: number) {
	return 1000 * (seconds + 60 * (minutes + 60 * hours));
}
