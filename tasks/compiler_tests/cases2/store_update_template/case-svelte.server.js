import * as $ from "svelte/internal/server";
import { count } from "./stores";
export default function App($$renderer) {
	var $$store_subs;
	$$renderer.push(`<button>inc</button> <button>pre inc</button> <button>dec</button>`);
	if ($$store_subs) $.unsubscribe_stores($$store_subs);
}
