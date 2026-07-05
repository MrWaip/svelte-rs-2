import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "";
	$$renderer.push(`<input${$.attr("value", (() => value)())}/>`);
}
