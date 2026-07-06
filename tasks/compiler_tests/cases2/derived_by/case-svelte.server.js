import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let doubled = $.derived(() => count * 2);
	$$renderer.push(`<p>0</p>`);
}
