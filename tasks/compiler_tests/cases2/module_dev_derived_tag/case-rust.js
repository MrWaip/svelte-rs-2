import * as $ from "svelte/internal/client";
let count = $.state(0);
const doubled = $.derived(() => $.get(count) * 2);
export function getDoubled() {
	return $.get(doubled);
}
export function increment() {
	$.update(count);
}
