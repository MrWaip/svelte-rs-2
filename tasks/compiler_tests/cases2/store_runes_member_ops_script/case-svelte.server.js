import * as $ from "svelte/internal/server";
import { obj } from "./stores";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		function go() {
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x = 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x += 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x -= 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x ??= 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x &&= 1);
			$.store_mutate($$store_subs ??= {}, "$obj", obj, $.store_get($$store_subs ??= {}, "$obj", obj).x ||= 1);
			$.store_get($$store_subs ??= {}, "$obj", obj).x++;
			$.store_get($$store_subs ??= {}, "$obj", obj).x--;
			++$.store_get($$store_subs ??= {}, "$obj", obj).x;
			--$.store_get($$store_subs ??= {}, "$obj", obj).x;
		}
		$$renderer.push(`<button>go</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
