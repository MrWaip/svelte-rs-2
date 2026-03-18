import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let state = $.state("visible");
	$.bind_property("visibilityState", "visibilitychange", $.document, ($$value) => $.set(state, $$value, true));
}
