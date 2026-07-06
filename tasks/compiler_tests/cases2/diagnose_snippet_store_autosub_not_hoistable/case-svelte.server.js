import * as $ from "svelte/internal/server";
import { page } from "$app/stores";
function defaultWrapWith($$renderer, mf) {
	mf($$renderer);
	$$renderer.push(`<!---->`);
}
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { wrapWith = defaultWrapWith } = $$props;
		function mf($$renderer) {
			if ($.store_get($$store_subs ??= {}, "$page", page).url) {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<b>x</b>`);
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		}
		$$renderer.push(`<div>`);
		wrapWith($$renderer, mf);
		$$renderer.push(`<!----></div>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
