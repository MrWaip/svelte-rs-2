import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let n = 0;
	function compute(x) {
		return x > 0;
	}
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_class("", void 0, { "active": compute(n) })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
