import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 1);
	let b = $.fallback($$props["b"], 2);
	let c = $$props["c"];
	$$renderer.push(`<p>${$.escape(a)}${$.escape(b)}${$.escape(c)}</p>`);
	$.bind_props($$props, {
		a,
		b,
		c
	});
}
