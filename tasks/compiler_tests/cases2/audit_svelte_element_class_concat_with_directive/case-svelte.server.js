import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tag = "div";
		let b = "two";
		let active = false;
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attr_class(`one ${$.stringify(b)}`, void 0, { "active": active })}`);
		}, () => {
			$$renderer.push(`x`);
		});
	});
}
