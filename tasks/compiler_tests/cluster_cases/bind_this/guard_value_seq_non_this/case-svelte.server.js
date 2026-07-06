import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	$$renderer.push(`<input${$.attr("value", (() => x)())}/>`);
}
