import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	const has_description = !!$$slots.description;
	if (has_description) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>has description</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
}
