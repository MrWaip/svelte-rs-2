import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let checked = $.fallback($$props["checked"], false);
		const bubbler = createBubbler();
		$$renderer.push(`<input${$.attr("checked", checked, true)} type="checkbox"/>`);
		$.bind_props($$props, { checked });
	});
}
