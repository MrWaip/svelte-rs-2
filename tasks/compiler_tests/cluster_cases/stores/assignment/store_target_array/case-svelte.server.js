import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const u = writable(1);
		const v = writable(2);
		const w = writable(3);
		function run() {
			(($$value) => {
				var $$array = $.to_array($$value, 3);
				$.store_set(u, $$array[0]);
				$.store_set(v, $$array[1]);
				$.store_set(w, $$array[2]);
			})([
				7,
				8,
				9
			]);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$u", u))}${$.escape($.store_get($$store_subs ??= {}, "$v", v))}${$.escape($.store_get($$store_subs ??= {}, "$w", w))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
