import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		p: [1, 2],
		q: [3, 4]
	}, $$array = $.to_array(tmp.p, 2), $$array_1 = $.to_array(tmp.q, 2), a = $.fallback($$props["a"], () => $$array[0], true), b = $.fallback($$props["b"], () => $$array[1], true), c = $.fallback($$props["c"], () => $$array_1[0], true), d = $.fallback($$props["d"], () => $$array_1[1], true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	$.bind_props($$props, {
		a,
		b,
		c,
		d
	});
}
