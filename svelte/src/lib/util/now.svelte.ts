let now = $state(Date.now());

// TODO: stop this maybe?
setInterval(() => {
	now = Date.now();
}, 1000);

export function timeNow() {
	return now;
}

export function durationNow(from: number) {
	return now - from;
}
