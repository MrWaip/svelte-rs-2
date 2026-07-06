import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let error;
	let id = $$props["id"];
	$: ({error} = make(id));
	let $$settled = true;
	let $$inner_renderer;
	function $$render_inner($$renderer) {
		Child($$renderer, {
			get value() {
				return $.store_get($$store_subs ??= {}, "$error", error);
			},
			set value($$value) {
				$.store_set(error, $$value);
				$$settled = false;
			}
		});
	}
	do {
		$$settled = true;
		$$inner_renderer = $$renderer.copy();
		$$render_inner($$inner_renderer);
	} while (!$$settled);
	$$renderer.subsume($$inner_renderer);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { id });
}
