import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		let b = "two";
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attr_class(`one ${$.stringify(b)}`)}`);
		}, () => {
			$$renderer.push(`x`);
		});
	});
}
