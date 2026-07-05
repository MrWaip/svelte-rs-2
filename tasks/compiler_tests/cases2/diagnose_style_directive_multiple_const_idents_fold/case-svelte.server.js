import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const c = "red";
	const w = "bold";
	const s = "16px";
	$$renderer.push(`<div${$.attr_style("", {
		color: c,
		"font-weight": w,
		"font-size": s
	})}></div>`);
}
