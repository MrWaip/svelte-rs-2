import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { method() {
		console.log("clicked");
	} };
	$$renderer.push(`<button>Click</button>`);
}
