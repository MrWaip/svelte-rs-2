import * as $ from "svelte/internal/server";
import { el } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$$renderer.push(`<div></div>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
