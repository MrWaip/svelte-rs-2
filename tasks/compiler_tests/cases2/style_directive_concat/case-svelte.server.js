import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let shade = "500";
	shade = "600";
	$$renderer.push(`<div${$.attr_style("", { color: `red-${$.stringify(shade)}` })}>Concat value</div>`);
}
