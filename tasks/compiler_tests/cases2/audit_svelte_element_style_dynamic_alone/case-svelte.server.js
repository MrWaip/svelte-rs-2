import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let s = "color: red";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_style(s)}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
