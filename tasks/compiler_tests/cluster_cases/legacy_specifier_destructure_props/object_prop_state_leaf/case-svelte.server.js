import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		a: 1,
		s: 2
	}, a = $.fallback($$props["a"], () => tmp.a, true), s = $.fallback($$props["s"], () => tmp.s, true);
	function inc() {
		a++;
		s++;
	}
	$$renderer.push(`<button>${$.escape(a)}${$.escape(s)}</button>`);
	$.bind_props($$props, { a });
}
