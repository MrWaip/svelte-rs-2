import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let ab, cd;
		const s = writable({
			"a-b": 1,
			"c d": 2
		});
		$: ({"a-b": ab, "c d": cd} = $.store_get($$store_subs ??= {}, "$s", s));
		$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
