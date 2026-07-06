import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let title = $.fallback($$props["title"], "");
		const bubbler = createBubbler();
		$$renderer.push(`<div${$.attr("title", title)}></div>`);
		$.bind_props($$props, { title });
	});
}
