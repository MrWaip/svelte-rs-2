import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const k = "z";
	let tmp = { z: 1 }, v = $.fallback($$props["v"], () => tmp[k], true);
	$$renderer.push(`<button>${$.escape(v)}</button>`);
	$.bind_props($$props, { v });
}
