import * as $ from "svelte/internal/server";
import { snippet } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	let arg = 1;
	$.store_get($$store_subs ??= {}, "$snippet", snippet)($$renderer, arg);
	$$renderer.push(`<!---->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
