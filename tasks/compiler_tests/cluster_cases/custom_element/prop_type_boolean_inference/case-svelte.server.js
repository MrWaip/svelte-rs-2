import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let red = $.fallback($$props["red"], false);
	red;
	$$renderer.push(`<div>hi</div>`);
	$.bind_props($$props, { red });
}
