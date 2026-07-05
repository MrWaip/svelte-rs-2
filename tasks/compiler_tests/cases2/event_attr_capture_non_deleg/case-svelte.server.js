import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handler() {
		console.log("scroll capture");
	}
	$$renderer.push(`<div>content</div>`);
}
