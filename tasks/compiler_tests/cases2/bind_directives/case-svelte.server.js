import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "";
	let name = "";
	$$renderer.push(`<input${$.attr("value", value)}/> <input${$.attr("value", name)}/> <input${$.attr("value", name)}/>`);
}
