import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tracking = false;
	$$renderer.push(`<p>${$.escape(tracking)}</p>`);
}
