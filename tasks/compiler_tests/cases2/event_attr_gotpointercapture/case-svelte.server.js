import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handler() {
		console.log("got pointer capture");
	}
	$$renderer.push(`<div>content</div>`);
}
