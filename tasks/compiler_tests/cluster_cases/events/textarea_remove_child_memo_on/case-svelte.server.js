import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = $.fallback($$props["value"], "");
		const bubbler = createBubbler();
		$$renderer.push(`<textarea>`);
		const $$body = $.escape(value);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.bind_props($$props, { value });
	});
}
