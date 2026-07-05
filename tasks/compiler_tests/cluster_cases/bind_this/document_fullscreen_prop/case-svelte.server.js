import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let fullscreen = $$props["fullscreen"];
	$$renderer.push(`<div>${$.escape(fullscreen)}</div>`);
	$.bind_props($$props, { fullscreen });
}
