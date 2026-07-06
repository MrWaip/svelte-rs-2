import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let obj = { el: null };
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`x`);
	});
}
