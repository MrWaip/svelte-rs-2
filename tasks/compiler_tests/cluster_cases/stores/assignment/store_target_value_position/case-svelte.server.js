import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const u = writable(1);
		const v = writable(2);
		let foo;
		let arr = [1, 2];
		function run() {
			foo = ((arr) => {
				var $$array = $.to_array(arr, 2);
				$.store_set(u, $$array[0]);
				$.store_set(v, $$array[1]);
				return arr;
			})(arr);
		}
		$$renderer.push(`<button>${$.escape(foo)}${$.escape($.store_get($$store_subs ??= {}, "$u", u))}${$.escape($.store_get($$store_subs ??= {}, "$v", v))}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
