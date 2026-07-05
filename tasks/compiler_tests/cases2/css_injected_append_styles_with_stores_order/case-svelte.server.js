import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
const $$css = {
	hash: "svelte-sw3owg",
	code: "p.svelte-sw3owg {color:red;}"
};
export default function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let store = writable(0);
		let count = 0;
		$$renderer.push(`<p class="svelte-sw3owg">${$.escape($.store_get($$store_subs ??= {}, "$store", store))} 0</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
