import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	let c = 0;
	$: x = x + c;
	$$renderer.push(`<input${$.attr("value", c)}/>`);
}
