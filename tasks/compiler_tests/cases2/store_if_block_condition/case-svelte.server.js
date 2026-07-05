import * as $ from "svelte/internal/server";
import { cond } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	if ($.store_get($$store_subs ??= {}, "$cond", cond)) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<p>visible</p>`);
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]-->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
