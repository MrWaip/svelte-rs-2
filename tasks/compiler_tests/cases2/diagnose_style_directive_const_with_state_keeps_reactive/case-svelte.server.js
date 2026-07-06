import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const transform = "translateY(1px)";
	let color = "red";
	$$renderer.push(`<div${$.attr_style("", {
		transform,
		color
	})}></div> <button>change</button>`);
}
