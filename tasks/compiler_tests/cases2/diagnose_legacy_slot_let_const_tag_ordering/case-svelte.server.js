import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $.fallback($$props["items"], () => [], true);
		Outer($$renderer, { $$slots: { cell: ($$renderer, { index, style }) => {
			const item = items[index];
			$$renderer.push(`<div slot="cell"${$.attr_style(style)}>${$.escape(item)}</div>`);
		} } });
		$.bind_props($$props, { items });
	});
}
