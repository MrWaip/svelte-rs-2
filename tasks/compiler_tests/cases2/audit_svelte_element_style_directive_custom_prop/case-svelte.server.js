import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let value = "red";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr_style("", { "--accent": value })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
