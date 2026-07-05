import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let color = "red";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_style("", [{}, { color }])}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
