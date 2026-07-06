import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function foo(node, x) {}
	let bar = 1;
	$$renderer.push(`<div></div>`);
}
