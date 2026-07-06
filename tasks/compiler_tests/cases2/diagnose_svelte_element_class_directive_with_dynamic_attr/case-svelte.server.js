import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let title = "hello";
	let active = false;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr("title", title)}${$.attr_class("", void 0, { "active": active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
