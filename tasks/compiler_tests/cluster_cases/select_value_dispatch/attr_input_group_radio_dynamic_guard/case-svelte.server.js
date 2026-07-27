import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let group = [];
	let v = "x";
	$$renderer.push(`<input type="radio"${$.attr("checked", group === v, true)}${$.attr("value", v)}/> <button>swap</button>`);
}
