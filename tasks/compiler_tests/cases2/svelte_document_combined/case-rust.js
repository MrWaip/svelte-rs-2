import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let el = $.state(null);
	function handleKeydown(e) {
		console.log("keydown", e.key);
	}
	$.event("keydown", $.document, handleKeydown);
	$.bind_active_element(($$value) => $.set(el, $$value, true));
}
