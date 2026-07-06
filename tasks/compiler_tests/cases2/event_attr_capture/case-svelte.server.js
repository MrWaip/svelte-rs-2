import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handler() {
		console.log("click capture");
	}
	$$renderer.push(`<button>Click</button>`);
}
