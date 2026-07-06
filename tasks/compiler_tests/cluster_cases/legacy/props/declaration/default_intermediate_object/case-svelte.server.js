import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let tmp = {}, a = $.fallback($$props["a"], () => $.fallback(tmp.p, () => ({}), true).a, true);
	$$renderer.push(`<button>${$.escape(a)}</button>`);
	$.bind_props($$props, { a });
}
