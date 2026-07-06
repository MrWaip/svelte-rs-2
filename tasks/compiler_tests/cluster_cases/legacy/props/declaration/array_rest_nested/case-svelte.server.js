import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.to_array(tmp), $$array_1 = $.to_array($$array.slice(1), 2), a = $.fallback($$props["a"], () => $$array[0], true), b = $.fallback($$props["b"], () => $$array_1[0], true), c = $.fallback($$props["c"], () => $$array_1[1], true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}${$.escape(c)}</button>`);
	$.bind_props($$props, {
		a,
		b,
		c
	});
}
