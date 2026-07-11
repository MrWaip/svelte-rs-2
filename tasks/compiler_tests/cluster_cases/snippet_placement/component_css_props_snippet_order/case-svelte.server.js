import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	let { data } = $$props;
	$.css_props($$renderer, true, { "--my-var": "baseline" }, () => {
		{
			function element($$renderer, { idx }) {
				$$renderer.push(`<div>${$.escape(idx)}</div>`);
			}
			Child($$renderer, {
				value: data,
				element,
				$$slots: { element: true }
			});
		}
	});
}
