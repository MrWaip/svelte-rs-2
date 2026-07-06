import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let store, doubled;
	let source = $$props["source"];
	$: store = source;
	$: doubled = $.store_get($$store_subs ??= {}, "$store", store) * 2;
	Child($$renderer, {
		value: store,
		other: doubled
	});
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { source });
}
