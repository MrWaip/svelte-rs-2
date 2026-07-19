import * as $ from "svelte/internal/client";
export function load(url) {
	return import(url);
}
