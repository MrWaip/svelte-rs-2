import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let val = 0;
	$$renderer.push(`<p>${$.escape(val)}</p>`);
}
