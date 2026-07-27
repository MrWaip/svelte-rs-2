import * as $ from "svelte/internal/client";
const cache = {};
export function fill(items) {
	items.forEach((item) => cache[item.id] = item);
}
