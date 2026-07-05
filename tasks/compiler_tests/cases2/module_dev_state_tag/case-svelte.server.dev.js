import * as $ from "svelte/internal/server";
let count = 0;
export function increment() {
	count++;
}
export function getCount() {
	return count;
}
