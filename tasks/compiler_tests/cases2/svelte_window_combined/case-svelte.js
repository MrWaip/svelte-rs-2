import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let scrollY = $.state(0);
	function handleResize() {
		console.log("resized");
	}
	$.event("resize", $.window, handleResize);
	$.bind_window_scroll("y", () => $.get(scrollY), ($$value) => $.set(scrollY, $$value, true));
}
