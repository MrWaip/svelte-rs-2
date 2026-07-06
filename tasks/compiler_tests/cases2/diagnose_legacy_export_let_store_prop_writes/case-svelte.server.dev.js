App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let store = $$props["store"];
		function clear() {
			$.store_set(store, undefined);
		}
		function bump() {
			$.store_mutate($$store_subs ??= {}, "$store", store, $.store_get($$store_subs ??= {}, "$store", store).x = 1);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 14, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$store", store)?.x)}</button>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 15, 0);
		$$renderer.push(`b</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { store });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
