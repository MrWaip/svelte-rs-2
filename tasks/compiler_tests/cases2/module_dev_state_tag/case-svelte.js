import * as $ from "svelte/internal/client";
let count = $.tag($.state(0), "count");
export function increment() {
	$.update(count);
}
export function getCount() {
	return $.get(count);
}
