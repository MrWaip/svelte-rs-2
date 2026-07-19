import * as $ from "svelte/internal/server";
export function load(url) {
	return import(url);
}
