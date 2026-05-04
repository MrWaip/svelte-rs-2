import { state } from 'svelte';

export let count = state(0);

export function increment() {
	count.set(count.get() + 1);
}
