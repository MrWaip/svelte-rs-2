import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let store = $$props["store"];
	const { items } = store;
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$items", items).list);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<div>${$.escape(item)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { store });
}
