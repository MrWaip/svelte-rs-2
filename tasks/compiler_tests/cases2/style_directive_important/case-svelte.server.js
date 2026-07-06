import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let color = "red";
	let bg = "blue";
	$$renderer.push(`<div${$.attr_style("", [{ color }, { "background-color": bg }])}>Important</div>`);
}
