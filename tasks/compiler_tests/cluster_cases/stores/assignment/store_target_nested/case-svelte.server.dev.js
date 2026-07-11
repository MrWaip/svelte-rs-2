App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const a = writable(1);
		const b = writable(2);
		const obj = {
			p: [10],
			q: { inner: 20 }
		};
		function run() {
			((obj) => {
				var $$array = $.to_array(obj.p, 1);
				$.store_set(a, $$array[0]);
				$.store_set(b, obj.q.inner);
			})(obj);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$a", a))}${$.escape($.store_get($$store_subs ??= {}, "$b", b))}</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
