import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$slots = $.sanitize_slots($$props);
	$$renderer.push(`<div><!--[-->`);
	$.slot($$renderer, $$props, "title", {}, null);
	$$renderer.push(`<!--]--> `);
	if ($$slots.description) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<hr/> <!--[-->`);
		$.slot($$renderer, $$props, "description", {}, null);
		$$renderer.push(`<!--]-->`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--></div>`);
}
