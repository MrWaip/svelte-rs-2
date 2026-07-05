import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let checked = false;
	$$renderer.push(`<input type="checkbox"${$.attr("checked", (() => checked)(), true)}/>`);
}
