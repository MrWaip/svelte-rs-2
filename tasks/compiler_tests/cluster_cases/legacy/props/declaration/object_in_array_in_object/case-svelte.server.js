import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = { outer: [{ inner: 1 }] }, $$array = $.to_array(tmp.outer, 1), inner = $.fallback($$props["inner"], () => $$array[0].inner, true);
	$$renderer.push(`<button>${$.escape(inner)}</button>`);
	$.bind_props($$props, { inner });
}
