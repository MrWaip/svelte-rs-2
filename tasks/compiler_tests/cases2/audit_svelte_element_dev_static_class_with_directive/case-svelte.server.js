import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let active = false;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("foo", void 0, { "active": active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
