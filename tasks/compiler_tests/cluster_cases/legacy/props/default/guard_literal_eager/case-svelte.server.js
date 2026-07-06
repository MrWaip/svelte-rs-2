import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $.fallback($$props["x"], 5);
	$$renderer.push(`<p>${$.escape(x)}</p>`);
	$.bind_props($$props, { x });
}
