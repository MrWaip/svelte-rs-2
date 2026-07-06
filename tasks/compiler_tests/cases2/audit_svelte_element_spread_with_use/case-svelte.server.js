import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let props = { id: "x" };
	function action(node) {}
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({ ...props })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
