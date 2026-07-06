import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let message = "hello";
	$$renderer.push(`<div>content</div>`);
}
