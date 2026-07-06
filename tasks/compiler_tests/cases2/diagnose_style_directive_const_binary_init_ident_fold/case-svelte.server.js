import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const w = "12" + "px";
	$$renderer.push(`<div${$.attr_style("", { width: w })}></div>`);
}
