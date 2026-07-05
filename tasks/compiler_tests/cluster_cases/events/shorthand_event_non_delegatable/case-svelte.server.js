import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let onmouseenter = $.fallback($$props["onmouseenter"], undefined);
	$$renderer.push(`<div></div>`);
	$.bind_props($$props, { onmouseenter });
}
