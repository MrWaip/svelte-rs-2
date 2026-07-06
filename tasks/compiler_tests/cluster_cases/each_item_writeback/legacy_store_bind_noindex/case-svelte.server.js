import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let items = $$props["items"];
	const { list } = items;
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like($.store_get($$store_subs ??= {}, "$list", list));
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			Child($$renderer, {
				get value() {
					return item;
				},
				set value($$value) {
					item = $$value;
					$$settled = false;
				}
			});
		}
		$$renderer.push(`<!--]-->`);
	}
	do {
		$$settled = true;
		$$inner_renderer = $$renderer.copy();
		$$render_inner($$inner_renderer);
	} while (!$$settled);
	$$renderer.subsume($$inner_renderer);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { items });
}
