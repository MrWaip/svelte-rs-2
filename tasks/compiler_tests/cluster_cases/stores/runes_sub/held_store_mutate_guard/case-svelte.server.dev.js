App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const source = writable({ bar: 0 });
		const held = $.derived(() => source);
		function bump() {
			$.store_mutate($$store_subs ??= {}, "$held", held, $.store_get($$store_subs ??= {}, "$held", held()).bar = 6);
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$held", held()).bar)}</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`bump</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
