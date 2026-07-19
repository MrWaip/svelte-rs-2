import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let label = "hi";
	$$renderer.push(`<p>hi</p>`);
}
