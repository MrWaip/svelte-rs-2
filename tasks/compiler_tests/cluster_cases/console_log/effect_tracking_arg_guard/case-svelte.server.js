import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function log() {
		console.log(false);
	}
	$$renderer.push(`<button>go</button>`);
}
