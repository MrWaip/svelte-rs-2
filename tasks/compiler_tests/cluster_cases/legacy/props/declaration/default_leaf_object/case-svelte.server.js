import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {}, a = $.fallback($$props["a"], () => $.fallback(tmp.a, 10), true), b = $.fallback($$props["b"], () => $.fallback(tmp.b, 20), true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	$.bind_props($$props, {
		a,
		b
	});
}
