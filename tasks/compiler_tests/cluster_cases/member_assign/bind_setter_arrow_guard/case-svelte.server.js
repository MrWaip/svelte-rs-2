import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: 0 };
	let value = 0;
	$$renderer.push(`<input${$.attr("value", (() => value)())}/>`);
}
