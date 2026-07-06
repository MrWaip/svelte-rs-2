import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let a = "a";
	let b = "b";
	let active = false;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class($.clsx([a, b]), void 0, { "active": active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
