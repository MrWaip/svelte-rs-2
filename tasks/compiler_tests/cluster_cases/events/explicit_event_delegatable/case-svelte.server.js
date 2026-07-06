import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let onclick = $.fallback($$props["onclick"], undefined);
	$$renderer.push(`<div></div>`);
	$.bind_props($$props, { onclick });
}
