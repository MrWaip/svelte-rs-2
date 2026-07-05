import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let isActive = true;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("", void 0, { "active": isActive })}`);
	}, () => {
		$$renderer.push(`content`);
	});
}
