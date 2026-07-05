import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let value;
	let store = $$props["store"];
	$: ({value} = store);
	$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$value", value))}</p>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { store });
}
