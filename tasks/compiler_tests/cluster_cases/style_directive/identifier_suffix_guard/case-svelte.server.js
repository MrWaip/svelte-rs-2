import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	$$renderer.push(`<button>go</button> <div${$.attr_style("", { left: `${$.stringify(x)}px` })}></div>`);
}
