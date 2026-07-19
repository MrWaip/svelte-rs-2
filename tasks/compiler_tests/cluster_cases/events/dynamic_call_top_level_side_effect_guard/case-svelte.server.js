import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let n = 0;
	function makeHandler() {
		return () => n++;
	}
	$$renderer.push(`<button>go</button>`);
}
