let count = $state(0);

export function increment() {
	count++;
}

export function getCount() {
	return count;
}
