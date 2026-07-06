import * as $ from "svelte/internal/server";
import { obj } from "./stores";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let key = "x";
		function go() {
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj)["k"] = 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj)[key] = 2);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj)["k"] += 1);
			$.store_get($$store_subs ??= {}, "$obj", obj)[key]++;
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj)["k"] ??= 5);
		}
		$$renderer.push(`<button>go</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
