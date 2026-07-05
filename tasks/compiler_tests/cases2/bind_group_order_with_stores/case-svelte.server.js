import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let metrics = writable([
			1,
			2,
			3
		]);
		let group = [];
		let total = $.derived(() => $.store_get($$store_subs ??= {}, "$metrics", metrics).length);
		$$renderer.push(`<input type="radio"${$.attr("checked", group === "a", true)} value="a"/> <input type="radio"${$.attr("checked", group === "b", true)} value="b"/> <p>${$.escape(total())}</p>`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
