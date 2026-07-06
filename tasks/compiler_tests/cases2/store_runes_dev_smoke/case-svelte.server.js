import * as $ from "svelte/internal/server";
import { obj, count } from "./stores";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function go() {
			$.store_set(count, 1);
			$.update_store($$store_subs ??= {}, "$count", count);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x = 1);
			$.store_get($$store_subs ??= {}, "$obj", obj).x++;
		}
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$count", count))}</p> <p>${$.escape($.store_get($$store_subs ??= {}, "$obj", obj).x)}</p> <button>go</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
