import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let group = [];
	let v = "x";
	$$renderer.push(`<input type="checkbox"${$.attr("checked", group.includes(v), true)}${$.attr("value", v)}/> <button>swap</button>`);
}
