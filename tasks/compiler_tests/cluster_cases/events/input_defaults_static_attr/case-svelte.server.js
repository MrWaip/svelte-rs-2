import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let checked = $.fallback($$props["checked"], false);
		const bubbler = createBubbler();
		const u = "http://x";
		$$renderer.push(`<input${$.attr("checked", checked, true)}${$.attr("formaction", u)} type="checkbox"/>`);
		$.bind_props($$props, { checked });
	});
}
