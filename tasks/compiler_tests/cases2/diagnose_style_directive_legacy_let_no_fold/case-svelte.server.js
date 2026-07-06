import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 1;
	$$renderer.push(`<div${$.attr_style("", { opacity: x })}></div> <button>bump</button>`);
}
