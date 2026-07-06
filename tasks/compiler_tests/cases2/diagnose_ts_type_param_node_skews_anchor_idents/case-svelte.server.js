import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let action;
	let x = $.fallback($$props["x"], false);
	$$renderer.push(`<div>`);
	if (x) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`a`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></div>`);
	$.bind_props($$props, { x });
}
