import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function onFocus() {}
	function onKey() {}
	$$renderer.push(`<div></div>`);
}
