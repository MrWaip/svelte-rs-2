import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	function action(node, opts) {}
	let opts = { x: 1 };
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`x`);
	});
}
