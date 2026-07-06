import * as $ from "svelte/internal/server";
export const M = 1;
export default function App($$renderer, $$props) {
	let a = $.fallback($$props["a"], 0);
	let b = $.fallback($$props["b"], "");
	$: b = b || (a ? "x" : "y");
	$$renderer.push(`<p>${$.escape(b)}</p>`);
	$.bind_props($$props, {
		a,
		b
	});
}
