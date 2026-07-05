import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handleKeydown(e) {
		console.log("keydown", e.key);
	}
	function handleKeyup(e) {
		console.log("keyup");
	}
}
