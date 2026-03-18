import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handler() {
		console.log("touch move");
	}
	$.event("touchmove", $.window, handler, void 0, true);
}
