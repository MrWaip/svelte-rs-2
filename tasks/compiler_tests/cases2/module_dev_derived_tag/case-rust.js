import * as $ from "svelte/internal/client";
let count = $.tag($.state(0), "count");
const doubled = $.tag($.derived(() => $.get(count) * 2), "doubled");
export function getDoubled() {
	return $.get(doubled);
}
export function increment() {
	$.update(count);
}
