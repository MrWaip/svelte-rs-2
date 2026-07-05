import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	function make() {
		return () => count++;
	}
	$$renderer.push(`<div${$.attr("title", count)}></div>`);
}
