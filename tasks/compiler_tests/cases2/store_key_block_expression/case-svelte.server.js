import * as $ from "svelte/internal/server";
import { token } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$$renderer.push(`<!---->`);
	{
		$$renderer.push(`<p>cycle</p>`);
	}
	$$renderer.push(`<!---->`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
