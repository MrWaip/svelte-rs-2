import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {
		a: 1,
		b: 2
	}, x = $.fallback($$props["x"], () => tmp.a, true), y = $.fallback($$props["y"], () => tmp.b, true);
	$$renderer.push(`<button>${$.escape(x)}${$.escape(y)}</button>`);
	$.bind_props($$props, {
		x,
		y
	});
}
