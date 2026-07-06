import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	let w = 0;
	$$renderer.push(`<button>go</button> <div${$.attr_style("", { left: `${$.stringify(Math.min(x + 3, w - 10))}px` })}></div>`);
}
