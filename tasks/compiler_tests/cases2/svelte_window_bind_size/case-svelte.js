import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let w = $.state(0);
	let h = $.state(0);
	$.bind_window_size("innerWidth", ($$value) => $.set(w, $$value, true));
	$.bind_window_size("innerHeight", ($$value) => $.set(h, $$value, true));
}
