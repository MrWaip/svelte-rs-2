import * as $ from "svelte/internal/server";
import { pick } from "./pick";
import { count } from "./count";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let kind = $$props["kind"];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(pick(kind, $.store_get($$store_subs ??= {}, "$count", count)));
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<div>${$.escape(item)}</div>`);
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { kind });
	});
}
