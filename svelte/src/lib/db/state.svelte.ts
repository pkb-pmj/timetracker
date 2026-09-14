export let dbState = $state({
	db: 0,
});

export function depends<T>(key: keyof typeof dbState, value: T) {
	dbState[key];
	return value;
}

export function invalidate(key: keyof typeof dbState) {
	dbState[key]++;
}
