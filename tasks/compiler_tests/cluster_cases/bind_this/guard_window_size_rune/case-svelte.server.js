import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let w = 0;
	$$renderer.push(`<div>${$.escape(w)}</div>`);
}
