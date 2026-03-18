import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let el = $.state(null);
	$.bind_active_element(($$value) => $.set(el, $$value, true));
}
