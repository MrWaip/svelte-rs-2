import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = [
		1,
		2,
		3
	], $$array = $.to_array(tmp, 3), a = $.fallback($$props["a"], () => $$array[0], true), c = $.fallback($$props["c"], () => $$array[2], true);
	$$renderer.push(`<button>${$.escape(a)}${$.escape(c)}</button>`);
	$.bind_props($$props, {
		a,
		c
	});
}
