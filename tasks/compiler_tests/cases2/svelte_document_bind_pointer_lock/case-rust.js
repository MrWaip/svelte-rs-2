import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let el = $.state(null);
	$.bind_property("pointerLockElement", "pointerlockchange", $.document, ($$value) => $.set(el, $$value, true));
}
