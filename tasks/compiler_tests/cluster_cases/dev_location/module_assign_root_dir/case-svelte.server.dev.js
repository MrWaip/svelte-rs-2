import * as $ from "svelte/internal/server";
const cache = {};
export function fill(items) {
	items.forEach((item) => cache[item.id] = item);
}
