import * as $ from "svelte/internal/server";
import { name } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$$renderer.push(`<input${$.attr("value", $.store_get($$store_subs ??= {}, "$name", name))}/>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
