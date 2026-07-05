import * as $ from "svelte/internal/server";
import { items } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$items", items));
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<p>${$.escape(item)}</p>`);
	}
	$$renderer.push(`<!--]-->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
