import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 0;
	function handler() {
		n++;
	}
	$$renderer.push(`<button>x</button>`);
}
