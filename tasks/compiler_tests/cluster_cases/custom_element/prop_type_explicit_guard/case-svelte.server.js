import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let count = $.fallback($$props["count"], 0);
	count;
	$$renderer.push(`<div>hi</div>`);
	$.bind_props($$props, { count });
}
