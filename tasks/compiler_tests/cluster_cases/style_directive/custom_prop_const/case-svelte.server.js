import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let alpha = 1;
	$$renderer.push(`<div${$.attr_style("", { "--css-variable": "rgba(0, 0, 0, 1)" })}></div>`);
}
