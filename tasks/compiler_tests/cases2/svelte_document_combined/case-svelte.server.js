import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let el = null;
	function handleKeydown(e) {
		console.log("keydown", e.key);
	}
}
