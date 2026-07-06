import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let title = $.fallback($$props["title"], undefined);
	$$renderer.push(`<div${$.attr("title", title)}></div>`);
	$.bind_props($$props, { title });
}
