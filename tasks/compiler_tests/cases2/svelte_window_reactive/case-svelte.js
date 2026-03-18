import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let innerWidth = $.state(0);
	let innerHeight = $.state(0);
	let outerWidth = $.state(0);
	let outerHeight = $.state(0);
	let devicePixelRatio = $.state(1);
	$.bind_window_size("innerWidth", ($$value) => $.set(innerWidth, $$value, true));
	$.bind_window_size("innerHeight", ($$value) => $.set(innerHeight, $$value, true));
	$.bind_window_size("outerWidth", ($$value) => $.set(outerWidth, $$value, true));
	$.bind_window_size("outerHeight", ($$value) => $.set(outerHeight, $$value, true));
	$.bind_property("devicePixelRatio", "resize", $.window, ($$value) => $.set(devicePixelRatio, $$value, true));
}
