import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const H = 8;
	let w = 0;
	setTimeout(() => {
		w = 10;
	});
	$$renderer.push(`<div${$.attr_style("", {
		height: "8px",
		width: `${$.stringify(w)}px`
	})}></div>`);
}
