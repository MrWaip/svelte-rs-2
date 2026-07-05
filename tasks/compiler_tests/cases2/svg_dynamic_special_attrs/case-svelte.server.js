import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = 1;
	let disabled = false;
	$$renderer.push(`<rect${$.attr("value", value)}${$.attr("disabled", disabled, true)}></rect>`);
}
