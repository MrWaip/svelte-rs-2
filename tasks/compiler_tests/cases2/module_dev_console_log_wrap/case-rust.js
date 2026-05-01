import * as $ from "svelte/internal/client";
let count = $.tag($.state(0), "count");
export function increment() {
	$.update(count);
}
$.user_effect(() => {
	console.log(...$.log_if_contains_state("log", "count changed:", $.get(count)));
});
