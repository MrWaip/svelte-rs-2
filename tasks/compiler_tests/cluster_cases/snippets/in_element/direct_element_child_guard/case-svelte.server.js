import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function foo($$renderer) {
		$$renderer.push(`<b>hi</b>`);
	}
	$$renderer.push(`<div></div>`);
}
