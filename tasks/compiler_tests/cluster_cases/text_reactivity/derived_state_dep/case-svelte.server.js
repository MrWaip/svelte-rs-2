import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let s = 0;
	const x = $.derived(() => s + 1);
	$$renderer.push(`<h1>1</h1>`);
}
