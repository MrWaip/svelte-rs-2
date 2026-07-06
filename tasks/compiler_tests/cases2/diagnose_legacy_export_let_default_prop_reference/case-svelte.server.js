import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let offsetX = $.fallback($$props["offsetX"], "");
	let paddingX = $.fallback($$props["paddingX"], offsetX);
	$$renderer.push(`<div>${$.escape(paddingX)}</div>`);
	$.bind_props($$props, {
		offsetX,
		paddingX
	});
}
