App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
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
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
