import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handleKeydown(e) {
		console.log("keydown", e.key);
	}
	$.event("keydown", $.document, handleKeydown);
}
