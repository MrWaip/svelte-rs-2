import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attr("title", [function(x) {
		return x;
	}])}></div>`);
}
