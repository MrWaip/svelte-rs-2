import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const a = writable(1);
		let rest;
		let arr = [
			1,
			2,
			3
		];
		function run() {
			((arr) => {
				var $$array = $.to_array(arr);
				$.store_set(a, $$array[0]);
				rest = $$array.slice(1);
			})(arr);
		}
		$$renderer.push(`<button>${$.escape($.store_get($$store_subs ??= {}, "$a", a))}${$.escape(rest.length)}</button>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
