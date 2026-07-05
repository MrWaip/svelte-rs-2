import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function handleMove() {
		console.log("move");
	}
	$$renderer.push(`<div>Touch</div>`);
}
