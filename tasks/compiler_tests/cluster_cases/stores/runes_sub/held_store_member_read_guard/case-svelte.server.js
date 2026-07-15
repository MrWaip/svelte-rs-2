import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const source = writable({ foo: 0 });
		const held = $.derived(() => source);
		const value = $.derived(() => $.store_get($$store_subs ??= {}, "$held", held()).foo);
		$$renderer.push(`<p>${$.escape(value())}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
