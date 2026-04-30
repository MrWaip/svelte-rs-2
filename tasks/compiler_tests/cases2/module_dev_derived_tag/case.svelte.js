let count = $state(0);
const doubled = $derived(count * 2);

export function getDoubled() {
	return doubled;
}

export function increment() {
	count++;
}
