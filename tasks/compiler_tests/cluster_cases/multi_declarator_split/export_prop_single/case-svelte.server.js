import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 1);
	$$renderer.push(`<p>${$.escape(a)}</p>`);
	$.bind_props($$props, { a });
}
