import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let active = false;
	let color = "red";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("", void 0, { "active": active })}${$.attr_style("", { color })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
