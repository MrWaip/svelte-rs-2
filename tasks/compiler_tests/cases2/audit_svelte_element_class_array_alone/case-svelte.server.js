import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let a = "a";
	let b = "b";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class($.clsx([a, b]))}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
