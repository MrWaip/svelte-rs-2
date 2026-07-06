import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let scrollY = $$props["scrollY"];
	$$renderer.push(`<div>${$.escape(scrollY)}</div>`);
	$.bind_props($$props, { scrollY });
}
