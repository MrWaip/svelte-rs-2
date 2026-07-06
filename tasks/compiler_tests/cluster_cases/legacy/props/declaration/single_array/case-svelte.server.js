import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = [1, 2], $$array = $.to_array(tmp, 2), a = $.fallback($$props["a"], () => $$array[0], true), b = $.fallback($$props["b"], () => $$array[1], true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(b)}</button>`);
	$.bind_props($$props, {
		a,
		b
	});
}
