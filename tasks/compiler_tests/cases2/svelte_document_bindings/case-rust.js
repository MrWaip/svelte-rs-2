import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let el = $.state(null);
	let state = $.state("visible");
	$.bind_active_element(($$value) => $.set(el, $$value, true));
	$.bind_property("fullscreenElement", "fullscreenchange", $.document, ($$value) => $.set(el, $$value, true));
	$.bind_property("pointerLockElement", "pointerlockchange", $.document, ($$value) => $.set(el, $$value, true));
	$.bind_property("visibilityState", "visibilitychange", $.document, ($$value) => $.set(state, $$value, true));
}
