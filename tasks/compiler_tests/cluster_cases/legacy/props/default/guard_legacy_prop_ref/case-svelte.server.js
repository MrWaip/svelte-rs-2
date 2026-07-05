import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 1);
	let b = $.fallback($$props["b"], a);
	$$renderer.push(`<p>${$.escape(a)}${$.escape(b)}</p>`);
	$.bind_props($$props, {
		a,
		b
	});
}
