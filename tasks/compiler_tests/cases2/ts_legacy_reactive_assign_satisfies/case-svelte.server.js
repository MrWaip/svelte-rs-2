import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let doubled;
	let count = 0;
	$: doubled = count * 2;
	$$renderer.push(`<p>${$.escape(doubled)}</p>`);
}
