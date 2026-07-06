import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let doubled;
	let x = $.fallback($$props["x"], 1);
	$: doubled = x * 2;
	$$renderer.push(`<p>${$.escape(doubled)}</p>`);
	$.bind_props($$props, { x });
}
