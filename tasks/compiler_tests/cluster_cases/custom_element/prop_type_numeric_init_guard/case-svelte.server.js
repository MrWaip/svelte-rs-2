import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let n = $.fallback($$props["n"], 0);
	n;
	$$renderer.push(`<div>hi</div>`);
	$.bind_props($$props, { n });
}
