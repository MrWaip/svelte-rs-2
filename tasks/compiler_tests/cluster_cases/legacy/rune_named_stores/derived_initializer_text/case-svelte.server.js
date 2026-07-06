import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let count = $.fallback($$props["count"], 0);
	let multiplier = $.fallback($$props["multiplier"], 2);
	let doubled = $.store_get($$store_subs ??= {}, "$derived", derived)(count * multiplier);
	let summary = $.store_get($$store_subs ??= {}, "$derived", derived)("x:" + count);
	$$renderer.push(`<p>doubled=${$.escape(doubled)}, summary=${$.escape(summary)}</p>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, {
		count,
		multiplier
	});
}
