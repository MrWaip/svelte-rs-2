import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let v;
		const k = "z";
		const s = writable({ z: 1 });
		$: ({[k]: v} = $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<button>${$.escape(v)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
