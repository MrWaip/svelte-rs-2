App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { count } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function go() {
			$.store_set(count, 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) + 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) - 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) * 2);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) / 2);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) % 3);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) ** 2);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) && 5);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) || 5);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) ?? 5);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) & 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) | 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) ^ 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) << 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) >> 1);
			$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) >>> 1);
			$.update_store($$store_subs ??= {}, "$count", count);
			$.update_store($$store_subs ??= {}, "$count", count, -1);
			$.update_store_pre($$store_subs ??= {}, "$count", count);
			$.update_store_pre($$store_subs ??= {}, "$count", count, -1);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 26, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
