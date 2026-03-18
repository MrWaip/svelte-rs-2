import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handleKeydown(e) {
		console.log("keydown");
	}
	function handleKeyup(e) {
		console.log("keyup");
	}
	$.event("keydown", $.document, handleKeydown);
	$.event("keyup", $.document, handleKeyup);
}
