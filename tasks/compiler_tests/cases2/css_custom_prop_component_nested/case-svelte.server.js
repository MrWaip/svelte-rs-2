import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let color = "red";
	$$renderer.push(`<div>`);
	$.css_props($$renderer, true, { "--color": color }, () => {
		Child($$renderer, {});
	});
	$$renderer.push(`</div>`);
}
