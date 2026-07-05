import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "x";
	$$renderer.push(`<input${$.attr("value", value)}/>`);
}
