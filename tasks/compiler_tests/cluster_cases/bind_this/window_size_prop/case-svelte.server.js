import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let width = $$props["width"];
	$$renderer.push(`<div>${$.escape(width)}</div>`);
	$.bind_props($$props, { width });
}
