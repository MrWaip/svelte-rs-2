App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
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
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 17, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$isLoading", isLoading) ? "wait" : "go")}-${$.escape(a)}-${$.escape(b)}-${$.escape(c)}-${$.escape(d)}</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			onSubmit,
			pair
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
