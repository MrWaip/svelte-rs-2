import * as $ from "svelte/internal/server";
import { count } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$.store_set(count, 5);
	$.update_store($$store_subs ??= {}, "$count", count);
	$.update_store_pre($$store_subs ??= {}, "$count", count);
	$.update_store($$store_subs ??= {}, "$count", count, -1);
	$.store_set(count, $.store_get($$store_subs ??= {}, "$count", count) + 10);
	$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$count", count))}</p>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
