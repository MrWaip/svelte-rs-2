App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable, derived } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const store = writable(0);
		let doubled = $.derived(() => $.store_get($$store_subs ??= {}, "$store", store) * 2);
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 7, 0);
		$$renderer.push(`${$.escape(doubled())} ${$.escape($.store_get($$store_subs ??= {}, "$store", store))}</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
