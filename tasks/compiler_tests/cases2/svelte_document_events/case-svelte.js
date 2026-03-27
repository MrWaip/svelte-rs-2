import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	function handleKeydown(e) {
		console.log("keydown", e.key);
	}
	function handleKeyup(e) {
		console.log("keyup");
	}
	$.event("keydown", $.document, handleKeydown);
	$.event("keyup", $.document, handleKeyup);
	$.event("keydown", $.document, $.once(handleKeydown), true);
}
