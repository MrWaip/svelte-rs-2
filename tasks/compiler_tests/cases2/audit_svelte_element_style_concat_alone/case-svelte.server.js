import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		let c = "red";
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attr_style(`color: ${$.stringify(c)}`)}`);
		}, () => {
			$$renderer.push(`x`);
		});
	});
}
