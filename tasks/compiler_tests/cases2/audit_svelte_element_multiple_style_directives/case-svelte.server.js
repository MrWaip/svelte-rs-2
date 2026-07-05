import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let color = "red";
	let bg = "white";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_style("", {
			color,
			background: bg
		})}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
