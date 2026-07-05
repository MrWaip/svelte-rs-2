import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	let x = $$props["x"];
	if ($$slots) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`a`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { x });
}
