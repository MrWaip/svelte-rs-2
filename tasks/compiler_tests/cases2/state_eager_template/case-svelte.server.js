import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = 1;
	let b = 2;
	$$renderer.push(`<p>${$.escape(a + b)}</p>`);
}
