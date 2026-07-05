import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function foo(node) {}
	$$renderer.push(`<div></div>`);
}
