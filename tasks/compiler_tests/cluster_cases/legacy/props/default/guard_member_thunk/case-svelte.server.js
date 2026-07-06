import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const obj = { v: 1 };
	let x = $.fallback($$props["x"], () => obj.v, true);
	$$renderer.push(`<p>${$.escape(x)}</p>`);
	$.bind_props($$props, { x });
}
