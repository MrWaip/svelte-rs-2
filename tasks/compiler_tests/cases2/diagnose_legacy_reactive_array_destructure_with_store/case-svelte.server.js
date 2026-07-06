import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	var $$store_subs;
	let run, isLoading, a, b, c, d;
	let onSubmit = $$props["onSubmit"];
	let pair = $$props["pair"];
	function withoutConcurrent(fn) {
		return [fn, { subscribe: () => () => {} }];
	}
	function go() {
		run();
	}
	$: [run, isLoading] = withoutConcurrent(onSubmit);
	$: [[a, b], [c, d]] = pair;
	$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "wait" : "go")}-${$.escape(a)}-${$.escape(b)}-${$.escape(c)}-${$.escape(d)}</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
	$.bind_props($$props, {
		onSubmit,
		pair
	});
}
