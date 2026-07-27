import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let a = "x";
	let g = void 0;
	$$renderer.push(`<input type="checkbox"${$.attr("checked", g, true)}${$.attr("value", a)}/>`);
}
