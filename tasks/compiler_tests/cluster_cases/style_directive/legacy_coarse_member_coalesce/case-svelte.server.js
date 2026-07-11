import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let obj;
	function upd(e) {
		obj = e;
	}
	$$renderer.push(`<div${$.attr_style("", { width: `${$.stringify((obj?.w || 0) + 40)}px` })}></div>`);
}
