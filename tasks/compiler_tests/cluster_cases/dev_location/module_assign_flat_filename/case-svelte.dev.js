import * as $ from "svelte/internal/client";
const cache = {};
export function fill(items) {
	items.forEach((item) => $.assign(cache, item.id, "=", item, "case.svelte.js:4:25"));
}
