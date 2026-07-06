import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj = { v: "x" };
	$$renderer.push(`<input${$.attr("value", obj.v)}/>`);
}
