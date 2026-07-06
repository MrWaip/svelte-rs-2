import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let nw = $$props["nw"];
	$$renderer.push(`<img alt="x"/>`);
	$.bind_props($$props, { nw });
}
