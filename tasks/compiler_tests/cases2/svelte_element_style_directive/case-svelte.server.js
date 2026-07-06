import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let col = "red";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_style("", { color: col })}`);
	}, () => {
		$$renderer.push(`content`);
	});
}
