import * as $ from "svelte/internal/server";
export async function load(cache, p) {
	return cache.items ??= await p;
}
