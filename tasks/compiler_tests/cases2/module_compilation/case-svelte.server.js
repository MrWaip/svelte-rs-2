import * as $ from "svelte/internal/server";
let count = 0;
const doubled = $.derived(() => count * 2);
export function increment() {
	count++;
}
export function getCount() {
	return count;
}
export function getDoubled() {
	return doubled();
}
