import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const op = .5;
	$$renderer.push(`<div${$.attr_style("", { opacity: op })}></div>`);
}
