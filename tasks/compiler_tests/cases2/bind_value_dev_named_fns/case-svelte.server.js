import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let state = "";
	$$renderer.push(`<input${$.attr("value", state)}/>`);
}
