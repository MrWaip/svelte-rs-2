import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 1);
	a;
	$$renderer.push(`<div>hi</div>`);
	$.bind_props($$props, { a });
}
