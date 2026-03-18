import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let isOnline = $.state(true);
	$.bind_online(($$value) => $.set(isOnline, $$value, true));
}
