import * as $ from "svelte/internal/server";
let count = 0;
const doubled = $.derived(() => count * 2);
export function getDoubled() {
	return doubled();
}
export function increment() {
	count++;
}
