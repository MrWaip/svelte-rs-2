import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		a: 1,
		b: 2
	}, a = $.fallback($$props["a"], () => tmp.a, true), b = $.fallback($$props["b"], () => tmp.b, true);
	function inc() {
		a++;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	$.bind_props($$props, { a });
}
