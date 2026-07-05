import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let enabled = true;
	$$renderer.push(`<div${$.attr("autofocus", enabled, true)}></div>`);
}
