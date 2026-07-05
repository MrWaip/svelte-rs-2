import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = [[1, 2], [3, 4]], $$array = $.to_array(tmp, 2), $$array_1 = $.to_array($$array[0], 2), $$array_2 = $.to_array($$array[1], 2), a = $.fallback($$props["a"], () => $$array_1[0], true), b = $.fallback($$props["b"], () => $$array_1[1], true), c = $.fallback($$props["c"], () => $$array_2[0], true), d = $.fallback($$props["d"], () => $$array_2[1], true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}${$.escape(d)}</button>`);
	$.bind_props($$props, {
		a,
		b,
		c,
		d
	});
}
