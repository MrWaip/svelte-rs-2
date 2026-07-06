import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let doubled;
	let width = 0;
	$: doubled = width * 2;
	$$renderer.push(`<p>${$.escape(doubled)}</p>`);
}
