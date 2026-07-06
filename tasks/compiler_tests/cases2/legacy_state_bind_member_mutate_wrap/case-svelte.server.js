import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { x: 1 };
	$$renderer.push(`<input${$.attr("value", obj.x)}/>`);
}
