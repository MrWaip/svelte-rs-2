import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const x = "lit";
	$$renderer.push(`<h1>lit</h1>`);
}
