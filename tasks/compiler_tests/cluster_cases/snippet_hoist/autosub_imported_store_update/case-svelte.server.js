import * as $ from "svelte/internal/server";
import { count } from "./store_mod.js";
export default function App($$renderer) {
	var $$store_subs;
	function foo($$renderer) {
		$$renderer.push(`<button></button>`);
	}
	foo($$renderer);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
