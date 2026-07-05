import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let active = false;
	let el = void 0;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("", void 0, { "active": active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
