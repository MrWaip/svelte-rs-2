import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "";
	function action(node) {}
	$$renderer.push(`<input${$.attr("value", value)}/>`);
}
