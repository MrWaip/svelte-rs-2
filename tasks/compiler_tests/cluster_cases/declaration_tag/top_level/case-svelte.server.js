import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const greeting = "hi";
	$$renderer.push(`<p>hi</p>`);
}
