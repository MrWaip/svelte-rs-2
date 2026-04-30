import * as $ from "svelte/internal/client";
let count = $.state(0);
export function increment() {
	$.update(count);
}
$.user_effect(() => {
	console.log("count changed:", $.get(count));
});
