import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const transform = "translateY(1px)";
	$$renderer.push(`<div${$.attr_style("", { transform })}></div>`);
}
