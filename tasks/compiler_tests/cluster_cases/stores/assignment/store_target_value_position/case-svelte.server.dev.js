App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const u = writable(1);
		const v = writable(2);
		let foo;
		let arr = [1, 2];
		function run() {
			foo = ((arr) => {
				var $$array = $.to_array(arr, 2);
				$.store_set(u, $$array[0]);
				$.store_set(v, $$array[1]);
				return arr;
			})(arr);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`${$.escape(foo)}${$.escape($.store_get($$store_subs ??= {}, "$u", u))}${$.escape($.store_get($$store_subs ??= {}, "$v", v))}</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
