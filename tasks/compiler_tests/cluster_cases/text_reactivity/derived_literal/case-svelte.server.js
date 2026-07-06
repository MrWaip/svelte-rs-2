import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const x = $.derived(() => 5);
	$$renderer.push(`<h1>5</h1>`);
}
