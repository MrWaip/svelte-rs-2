import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let watcherA = void 0;
		if (watcherA) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$watcherA", watcherA))} <button>remove</button>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<button>add</button>`);
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
