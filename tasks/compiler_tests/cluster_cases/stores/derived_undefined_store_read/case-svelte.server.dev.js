App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let store = undefined;
		let value = $.derived(() => $.store_get($$store_subs ??= {}, "$store", store));
		$$renderer.push(`<h1>`);
		$.push_element($$renderer, "h1", 6, 0);
		$$renderer.push(`${$.escape(value())}</h1>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
