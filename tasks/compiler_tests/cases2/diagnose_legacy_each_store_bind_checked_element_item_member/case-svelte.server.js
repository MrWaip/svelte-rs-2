import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let store = $$props["store"];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$store", store));
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		if (item.enabled) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<input type="checkbox"${$.attr("checked", item.enabled, true)}/>`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
	}
	$$renderer.push(`<!--]-->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { store });
}
