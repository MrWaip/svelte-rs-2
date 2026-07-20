import * as $ from "svelte/internal/server";
export const meta = { title: "x" };
export default function App($$renderer) {
	let one = 1;
	let doubled = 0;
	$: doubled = one * 2;
	$$renderer.push(`<input type="radio"${$.attr("checked", one === 1, true)}${$.attr("value", 1)}/> <input type="radio"${$.attr("checked", one === 2, true)}${$.attr("value", 2)}/> <span>${$.escape(doubled)}</span>`);
}
