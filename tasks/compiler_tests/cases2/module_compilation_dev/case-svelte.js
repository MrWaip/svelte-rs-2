import * as $ from "svelte/internal/client";
let count = $.tag($.state(0), "count");
const doubled = $.tag($.derived(() => $.get(count) * 2), "doubled");
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
	console.log(...$.log_if_contains_state("log", "count changed:", $.get(count)));
});
