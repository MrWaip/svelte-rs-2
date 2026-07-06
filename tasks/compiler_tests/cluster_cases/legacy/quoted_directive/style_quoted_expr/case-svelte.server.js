import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let c = "red";
	$$renderer.push(`<div${$.attr_style("", { color: c })}></div>`);
}
