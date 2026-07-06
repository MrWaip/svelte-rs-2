import * as $ from "svelte/internal/server";
import { count } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	function set() {
		$.store_set(count, 5);
	}
	$$renderer.push(`<button>set</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
