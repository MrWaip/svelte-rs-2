import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function foo(n) {
		return n + 1;
	}
	$$renderer.push(`<button>go</button> <div${$.attr_style("", { left: `${$.stringify(foo(x))}px` })}></div>`);
}
