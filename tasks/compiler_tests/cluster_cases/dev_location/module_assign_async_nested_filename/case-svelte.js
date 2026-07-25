import * as $ from "svelte/internal/client";
export async function load(cache, p) {
	return cache.items ??= await p;
}
