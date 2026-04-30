import * as $ from "svelte/internal/client";
let count = $.state(0);
const doubled = $.derived(() => $.get(count) * 2);
export function increment() {
	$.update(count);
}
export function getCount() {
	return $.get(count);
}
export function getDoubled() {
	return $.get(doubled);
}
$.user_effect(() => {
	console.log("count changed:", $.get(count));
});
