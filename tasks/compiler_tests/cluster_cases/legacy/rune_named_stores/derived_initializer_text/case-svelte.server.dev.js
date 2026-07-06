App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let count = $.fallback($$props["count"], 0);
		let multiplier = $.fallback($$props["multiplier"], 2);
		let doubled = $.store_get($$store_subs ??= {}, "$derived", derived)(count * multiplier);
		let summary = $.store_get($$store_subs ??= {}, "$derived", derived)("x:" + count);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 9, 0);
		$$renderer.push(`doubled=${$.escape(doubled)}, summary=${$.escape(summary)}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			count,
			multiplier
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
