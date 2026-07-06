import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let s = "color: red";
	let fs = "12px";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_style(s, { "font-size": fs })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
