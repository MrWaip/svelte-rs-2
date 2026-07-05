import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let prop = $$props["prop"];
		const items = writable([]);
		let show = true;
		if (show && $.store_get($$store_subs ??= {}, "$items", items).length > 0) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<p>shown ${$.escape(prop)}</p>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { prop });
	});
}
