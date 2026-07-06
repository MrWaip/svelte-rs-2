import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "rect";
	let active = false;
	$.element($$renderer, tag, () => {
		$$renderer.push(` xmlns="http://www.w3.org/2000/svg"${$.attr_class("", void 0, { "active": active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
