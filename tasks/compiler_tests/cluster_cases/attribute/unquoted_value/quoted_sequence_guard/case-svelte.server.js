import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let value = "x";
	$$renderer.push(`<div${$.attr("foo", `a${$.stringify(value)}`)}></div> <button>go</button>`);
}
