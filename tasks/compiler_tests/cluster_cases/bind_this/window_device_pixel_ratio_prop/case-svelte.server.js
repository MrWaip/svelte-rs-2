import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let dpr = $$props["dpr"];
	$$renderer.push(`<div>${$.escape(dpr)}</div>`);
	$.bind_props($$props, { dpr });
}
