import * as $ from "svelte/internal/server";
import { count } from "./stores";
export default function App($$renderer) {
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
	$$renderer.push(`<button>go</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
