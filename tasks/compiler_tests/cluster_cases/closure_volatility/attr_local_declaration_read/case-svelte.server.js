import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div${$.attr("title", [() => {
		let q = 1;
		return q;
	}])}></div>`);
}
