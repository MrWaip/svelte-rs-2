import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function getHandler() {
		return () => count++;
	}
	$$renderer.push(`<button>Click</button>`);
}
