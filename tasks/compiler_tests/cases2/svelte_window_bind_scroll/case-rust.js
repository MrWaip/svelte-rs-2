import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let scrollX = $.state(0);
	let scrollY = $.state(0);
	$.bind_window_scroll("x", () => $.get(scrollX), ($$value) => $.set(scrollX, $$value, true));
	$.bind_window_scroll("y", () => $.get(scrollY), ($$value) => $.set(scrollY, $$value, true));
}
