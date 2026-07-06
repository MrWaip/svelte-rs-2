import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		const value = writable({
			a: 1,
			b: 2
		});
		if ($.store_get($$store_subs ??= {}, "$value", value)) {
			$$renderer.push("<!--[0-->");
			const { a, b } = $.store_get($$store_subs ??= {}, "$value", value);
			Comp($$renderer, {
				x: a,
				b
			});
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	});
}
