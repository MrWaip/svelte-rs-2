App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const x = writable(1);
		const y = writable(2);
		function run() {
			(($$value) => {
				$.store_set(x, $$value.a);
				$.store_set(y, $$value.b);
			})({
				a: 9,
				b: 10
			});
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 10, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$x", x))}${$.escape($.store_get($$store_subs ??= {}, "$y", y))}</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
