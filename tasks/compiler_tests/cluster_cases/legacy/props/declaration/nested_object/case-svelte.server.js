import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		p: { a: 1 },
		q: { b: 2 }
	}, a = $.fallback($$props["a"], () => tmp.p.a, true), b = $.fallback($$props["b"], () => tmp.q.b, true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	$.bind_props($$props, {
		a,
		b
	});
}
