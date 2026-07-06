import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let one = 1;
	let two = 2;
	$$renderer.push(`<input type="radio"${$.attr("checked", one === 1, true)}${$.attr("value", 1)}/> <input type="radio"${$.attr("checked", two === 2, true)}${$.attr("value", 2)}/>`);
}
