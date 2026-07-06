App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { obj } from "./stores";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function go() {
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x = 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x += 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x ??= 5);
			$.store_get($$store_subs ??= {}, "$obj", obj).x++;
			++$.store_get($$store_subs ??= {}, "$obj", obj).x;
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 11, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
