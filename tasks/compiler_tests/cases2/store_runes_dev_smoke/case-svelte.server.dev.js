App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { obj, count } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function go() {
			$.store_set(count, 1);
			$.update_store($$store_subs ??= {}, "$count", count);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x = 1);
			$.store_get($$store_subs ??= {}, "$obj", obj).x++;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 10, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$count", count))}</p>`);
		$.pop_element();
		$$renderer.push(` <p>`);
		$.push_element($$renderer, "p", 11, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$obj", obj).x)}</p>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 12, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
