import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const s = writable(1);
		let plain;
		function run() {
			(($$value) => {
				var $$array = $.to_array($$value, 2);
				plain = $$array[0];
				$.store_set(s, $$array[1]);
			})([1, 2]);
		}
		$$renderer.push(`<button>${$.escape(plain)}${$.escape($.store_get($$store_subs ??= {}, "$s", s))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
