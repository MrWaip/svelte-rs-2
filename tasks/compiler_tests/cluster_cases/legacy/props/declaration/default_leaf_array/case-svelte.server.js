import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = [1], $$array = $.to_array(tmp, 2), a = $.fallback($$props["a"], () => $.fallback($$array[0], 10), true), b = $.fallback($$props["b"], () => $.fallback($$array[1], 20), true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	$.bind_props($$props, {
		a,
		b
	});
}
