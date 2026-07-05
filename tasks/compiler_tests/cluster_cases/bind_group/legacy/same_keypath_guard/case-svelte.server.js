import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let data = { a: 1 };
	$$renderer.push(`<input type="radio"${$.attr("checked", data.a === 1, true)}${$.attr("value", 1)}/> <input type="radio"${$.attr("checked", data.a === 2, true)}${$.attr("value", 2)}/>`);
}
