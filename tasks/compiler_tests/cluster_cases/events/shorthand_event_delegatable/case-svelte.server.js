import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let onkeydown = $.fallback($$props["onkeydown"], undefined);
	$$renderer.push(`<div></div>`);
	$.bind_props($$props, { onkeydown });
}
