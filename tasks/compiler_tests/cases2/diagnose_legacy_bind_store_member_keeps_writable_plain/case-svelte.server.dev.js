App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let config = $$props["config"];
		let address = writable(Object.assign({}, config.address));
		function reset() {
			$.store_set(address, { foo: 0 });
		}
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			Child($$renderer, {
				get value() {
					return $.store_get($$store_subs ??= {}, "$address", address).foo;
				},
				set value($$value) {
					$.store_mutate($$store_subs ??= {}, "$address", address, $.store_get($$store_subs ??= {}, "$address", address).foo = $$value);
					$$settled = false;
				}
			});
			$$renderer.push(`<!----> <button>`);
			$.push_element($$renderer, "button", 14, 0);
			$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$address", address).foo)}</button>`);
			$.pop_element();
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, { config });
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
