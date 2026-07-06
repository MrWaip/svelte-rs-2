import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let left, right, renamed, deep;
	let source = $$props["source"];
	$: ({left, right, alias: renamed, nested: {deep}} = source);
	$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$left", left))}-${$.escape($.store_get($$store_subs ??= {}, "$right", right))}-${$.escape(renamed)}-${$.escape(deep)}</p>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, { source });
}
