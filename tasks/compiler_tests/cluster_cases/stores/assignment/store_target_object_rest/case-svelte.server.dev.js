App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const a = writable(1);
		let rest;
		const obj = {
			$a: 1,
			c: 2,
			d: 3
		};
		function run() {
			$.store_set(a, obj.$a), rest = $.exclude_from_object(obj, ["$a"]);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$a", a))}${$.escape(rest.c)}</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
