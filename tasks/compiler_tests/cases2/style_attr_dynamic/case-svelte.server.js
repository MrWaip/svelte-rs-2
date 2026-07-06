import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let color = "red";
	function toggle() {
		color = "blue";
	}
	$$renderer.push(`<div${$.attr_style(`color: ${$.stringify(color)}; font-size: 14px`)}></div>`);
}
