import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let elRef = $.fallback($$props["elRef"], undefined);
	$$renderer.push(`<div></div>`);
	$.bind_props($$props, { elRef });
}
