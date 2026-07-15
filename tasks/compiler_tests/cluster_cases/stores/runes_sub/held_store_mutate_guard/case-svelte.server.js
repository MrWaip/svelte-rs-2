import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const source = writable({ bar: 0 });
		const held = $.derived(() => source);
		function bump() {
			$.store_mutate($$store_subs ??= {}, "$held", held, $.store_get($$store_subs ??= {}, "$held", held()).bar = 6);
		}
		$$renderer.push(`<p>${$.escape($.store_get($$store_subs ??= {}, "$held", held()).bar)}</p> <button>bump</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
