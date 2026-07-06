import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = $$props["x"];
	$$renderer.push(`<p>${$.escape(x)}</p>`);
	$.bind_props($$props, { x });
}
