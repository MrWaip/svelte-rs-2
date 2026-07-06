import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let size = "16px";
	size = "20px";
	$$renderer.push(`<div${$.attr_style("", {
		color: "red",
		"font-size": size
	})}>String value</div>`);
}
