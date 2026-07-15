import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { items } = $$props;
		$.css_props($$renderer, true, { "--my-var": "baseline" }, () => {
			{
				function element($$renderer, { idx }) {
					$$renderer.push(`<div>${$.escape(idx)}</div>`);
				}
				Child($$renderer, {
					disabled: !items.length,
					element,
					$$slots: { element: true }
				});
			}
		});
	});
}
