import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	function attachment(node) {}
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`x`);
	});
}
