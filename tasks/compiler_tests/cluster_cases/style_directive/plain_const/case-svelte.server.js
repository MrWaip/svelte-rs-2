import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let col = "red";
	$$renderer.push(`<div${$.attr_style("", { color: col })}></div>`);
}
