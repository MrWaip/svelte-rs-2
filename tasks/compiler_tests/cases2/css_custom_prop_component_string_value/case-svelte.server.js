import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div>`);
	$.css_props($$renderer, true, { "--color": "25%" }, () => {
		Child($$renderer, {});
	});
	$$renderer.push(`</div>`);
}
