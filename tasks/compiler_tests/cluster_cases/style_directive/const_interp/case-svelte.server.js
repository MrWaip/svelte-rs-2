import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let c = "0, 0, 0";
	$$renderer.push(`<div${$.attr_style("", { color: "rgb(0, 0, 0)" })}></div>`);
}
